use crate::domain::config::Config;
use crate::infra::config::ConfigStore;
use eyre::Result;

/// Initializes Kittynode with the default config
pub fn init_kittynode() -> Result<()> {
    let config = Config::default();
    ConfigStore::save(&config)?; // Explicitly use ConfigStore for persistence.
    Ok(())
}
