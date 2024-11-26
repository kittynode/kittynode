use crate::infra::config::ConfigStore;
use eyre::Result;

pub fn add_capability(capability: &str) -> Result<()> {
    let mut config = ConfigStore::load()?;
    if !config.capabilities.contains(&capability.to_string()) {
        config.capabilities.push(capability.to_string());
    }
    ConfigStore::save(&config)?;
    Ok(())
}
