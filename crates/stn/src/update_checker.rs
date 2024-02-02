use crate::stn_config::StnConfig;
use semver::Version;
use serde_json::Value as JsonValue;
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::constants;

pub struct UpdateChecker {
    stn_config_dir: PathBuf,
}

impl UpdateChecker {
    pub fn new(stn_config_dir: PathBuf) -> Self {
        UpdateChecker { stn_config_dir }
    }

    /// Main function to check for updates
    pub async fn check_for_updates(&self) -> Result<(), Box<dyn std::error::Error>> {
        let current_version = Version::parse(env!("CARGO_PKG_VERSION"))?;
        let mut config = StnConfig::load(&self.stn_config_dir)?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let twelve_hours = 43200;

        if now - config.last_update_check > twelve_hours || config.latest_version.is_none() {
            let latest_version = self.get_latest_release_version().await?;

            if let Some(latest_version) = latest_version {
                if latest_version > current_version {
                    self.display_update_message(&latest_version);
                }
                config.latest_version = Some(latest_version.to_string());
            }

            config.last_update_check = now;
            config.save(&self.stn_config_dir)?;
        } else if let Some(cached_latest_version_str) = &config.latest_version {
            let cached_latest_version = Version::parse(cached_latest_version_str)?;
            if cached_latest_version > current_version {
                self.display_update_message(&cached_latest_version);
            }
        }

        Ok(())
    }

    /// Display the update message
    fn display_update_message(&self, latest_version: &Version) {
        println!(
            "A new version of stn is available: {}! ヾ(•ω•`)o",
            latest_version
        );
        println!(
            "Visit https://github.com/{}/{}/releases to download it.\n",
            constants::GITHUB_REPO_USERNAME,
            constants::GITHUB_REPO_NAME
        );
    }

    /// API call to fetch the latest release version
    async fn get_latest_release_version(
        &self,
    ) -> Result<Option<Version>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            constants::GITHUB_REPO_USERNAME,
            constants::GITHUB_REPO_NAME,
        );

        let client = reqwest::Client::builder().user_agent("stn").build()?;

        let response_text = client.get(&url).send().await?.text().await?;
        let response = serde_json::from_str::<JsonValue>(&response_text)?;

        if let Some(tag_name) = response["tag_name"].as_str() {
            return Ok(Some(Version::parse(tag_name.trim_start_matches('v'))?));
        }

        Ok(None)
    }
}
