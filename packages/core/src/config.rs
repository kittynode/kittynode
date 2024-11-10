use crate::file::kittynode_path;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub capabilities: Vec<String>,
    pub remote_url: Option<String>,
}

impl Config {
    /// Loads the configuration from the TOML file.
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;
        if !config_path.exists() {
            return Ok(Config::default());
        }
        let toml_str = fs::read_to_string(config_path)?;
        let config = toml::from_str(&toml_str)?;
        Ok(config)
    }

    /// Saves the current configuration to the TOML file.
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let toml_str = toml::to_string(self)?;
        fs::write(config_path, toml_str)?;
        Ok(())
    }

    /// Returns the path to the configuration file.
    fn config_file_path() -> Result<PathBuf> {
        let mut path = kittynode_path()?;
        path.push("config.toml");
        Ok(path)
    }

    /// Adds a capability to the configuration.
    pub fn add_capability(&mut self, capability: &str) {
        if !self.capabilities.contains(&capability.to_string()) {
            self.capabilities.push(capability.to_string());
        }
    }

    /// Removes a capability from the configuration.
    pub fn remove_capability(&mut self, capability: &str) {
        if let Some(pos) = self.capabilities.iter().position(|x| x == capability) {
            self.capabilities.remove(pos);
        }
    }

    /// Retrieves all capabilities from the configuration.
    pub fn get_capabilities(&self) -> &[String] {
        &self.capabilities
    }

    /// Sets the remote url in the configuration.
    pub fn set_remote_url(&mut self, endpoint: Option<String>) {
        self.remote_url = endpoint;
    }

    /// Retrieves the remote url from the configuration.
    pub fn get_remote_url(&self) -> Option<&String> {
        self.remote_url.as_ref()
    }
}

/// Module-level function to add a capability.
pub fn add_capability(capability: &str) -> Result<()> {
    let mut config = Config::load()?;
    config.add_capability(capability);
    config.save()?;
    Ok(())
}

/// Module-level function to remove a capability.
pub fn remove_capability(capability: &str) -> Result<()> {
    let mut config = Config::load()?;
    config.remove_capability(capability);
    config.save()?;
    Ok(())
}

/// Module-level function to get all capabilities.
pub fn get_capabilities() -> Result<Vec<String>> {
    let config = Config::load()?;
    Ok(config.capabilities.clone())
}

/// Module-level function to set the remote url.
pub fn set_remote_url(endpoint: Option<String>) -> Result<()> {
    let mut config = Config::load()?;
    config.set_remote_url(endpoint);
    config.save()?;
    Ok(())
}

/// Module-level function to get the remote url.
pub fn get_remote_url() -> Result<Option<String>> {
    let config = Config::load()?;
    Ok(config.remote_url)
}
