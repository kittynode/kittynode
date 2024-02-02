use reqwest;
use semver::Version;
use serde_json::Value as JsonValue;
use std::{
    fs,
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
        let last_check = self.get_last_update_check()?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let cached_latest_version = self.get_cached_latest_version()?;
        let twelve_hours = 43200;

        // If it's been more than 12 hours since the last check, or the latest version is not cached, check for updates
        // with a real API call!
        if now - last_check > twelve_hours || cached_latest_version.is_none() {
            let latest_version = self.get_latest_release_version().await?;
            if let Some(latest_version) = latest_version {
                self.cache_latest_version(&latest_version)?;
                if latest_version > current_version {
                    self.display_update_message(&latest_version);
                }
            }
            self.set_last_update_check(now)?;
        }
        // Otherwise, display if they need to update based on the cached version
        else if let Some(cached_latest_version) = cached_latest_version {
            if cached_latest_version > current_version {
                self.display_update_message(&cached_latest_version);
            }
        }

        Ok(())
    }

    /// Display the update message
    fn display_update_message(&self, latest_version: &Version) {
        println!("A new version of stn is available: {}", latest_version);
        println!(
            "Visit https://github.com/{}/{}/releases to download it.",
            constants::GITHUB_REPO_USERNAME,
            constants::GITHUB_REPO_NAME
        );
    }

    // Get latest version from cache, if doesn't exist return None
    fn get_cached_latest_version(&self) -> Result<Option<Version>, std::io::Error> {
        let version_path = self.stn_config_dir.join(".latest_version");
        let content = fs::read_to_string(&version_path);

        match content {
            Ok(content) => {
                let trimmed_content = content.trim();
                if trimmed_content.is_empty() {
                    // If the file is empty, treat it as if the version is not cached
                    Ok(None)
                } else {
                    // Attempt to parse the version, mapping any parsing errors to std::io::Error
                    let parsed_version = Version::parse(trimmed_content).map_err(|err| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("Invalid version format: {}", err),
                        )
                    })?;
                    Ok(Some(parsed_version))
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                // If the file does not exist, treat it as if the version is not cached
                Ok(None)
            }
            Err(err) => {
                // Propagate other I/O errors
                Err(err)
            }
        }
    }

    /// Cache the latest version from the last check
    fn cache_latest_version(&self, version: &Version) -> Result<(), std::io::Error> {
        let version_path = self.stn_config_dir.join(".latest_version");
        fs::write(version_path, version.to_string().as_bytes())
    }

    /// Get the timestamp of the last update check
    fn get_last_update_check(&self) -> Result<u64, std::io::Error> {
        let last_check_path = self.stn_config_dir.join(".last_update_check");
        let content = fs::read_to_string(&last_check_path).unwrap_or_else(|_| "0".to_string());
        Ok(content.trim().parse().unwrap_or(0))
    }

    /// Updates the last update check
    fn set_last_update_check(&self, timestamp: u64) -> Result<(), std::io::Error> {
        let last_check_path = self.stn_config_dir.join(".last_update_check");
        fs::write(last_check_path, timestamp.to_string().as_bytes())
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
