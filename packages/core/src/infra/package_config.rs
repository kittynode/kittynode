use crate::domain::package::PackageConfig;
use crate::infra::file::kittynode_path;
use eyre::Result;
use serde_json;
use std::{fs, path::PathBuf};

pub struct PackageConfigStore;

impl PackageConfigStore {
    pub fn load(package_name: &str) -> Result<PackageConfig> {
        let config_path = Self::config_file_path(package_name)?;
        if !config_path.exists() {
            return Ok(PackageConfig::default());
        }
        let json_str = fs::read_to_string(config_path)?;
        let config = serde_json::from_str(&json_str)?;
        Ok(config)
    }

    pub fn save(package_name: &str, config: &PackageConfig) -> Result<()> {
        let config_path = Self::config_file_path(package_name)?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json_str = serde_json::to_string_pretty(config)?;
        fs::write(config_path, json_str)?;
        Ok(())
    }

    fn config_file_path(package_name: &str) -> Result<PathBuf> {
        let mut path = kittynode_path()?;
        path.push("packages");
        path.push(package_name);
        path.push("config.json");
        Ok(path)
    }
}
