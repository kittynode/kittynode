use semver::Version;
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::constants;

pub struct UpdateChecker {
    taiko_node_dir: PathBuf,
}

impl UpdateChecker {
    pub fn new(taiko_node_dir: PathBuf) -> Self {
        UpdateChecker { taiko_node_dir }
    }

    pub async fn check_for_updates(&self) -> Result<(), Box<dyn std::error::Error>> {
        let current_version = Version::parse(env!("CARGO_PKG_VERSION"))?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        // Removed the time check, so it always checks for updates
        // TODO: only make the update check each 12 hours, but also when a fresh install
        if let Some(latest_version) = self.get_latest_release_version().await? {
            if latest_version > current_version {
                println!("A new version of stn is available: {}", latest_version);
                println!(
                    "Visit https://github.com/{}/{}/releases to download it.",
                    constants::GITHUB_REPO_USERNAME,
                    constants::GITHUB_REPO_NAME
                );
                // Do not update the last check time unless the user updates the version
                // This way, the message will keep showing until they update
            } else {
                // Update the last check time only if the version is up-to-date
                self.set_last_update_check(now)?;
            }
        }

        Ok(())
    }

    fn set_last_update_check(&self, timestamp: u64) -> Result<(), std::io::Error> {
        let last_check_path = self.taiko_node_dir.join(".last_update_check");
        fs::write(last_check_path, timestamp.to_string().as_bytes())
    }

    async fn get_latest_release_version(
        &self,
    ) -> Result<Option<Version>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            constants::GITHUB_REPO_USERNAME,
            constants::GITHUB_REPO_NAME,
        );

        // Create a client to make the HTTP request
        let client = reqwest::Client::builder()
            .user_agent("stn") // Set a user-agent for your app
            .build()?;

        let response_text = client.get(&url).send().await?.text().await?;

        let response = serde_json::from_str::<serde_json::Value>(&response_text)?;

        if let Some(tag_name) = response["tag_name"].as_str() {
            return Ok(Some(Version::parse(tag_name.trim_start_matches('v'))?));
        }

        Ok(None)

        // let mocked_latest_version: &str = "2.0.0";

        // Ok(Some(Version::parse(mocked_latest_version)?))
    }
}
