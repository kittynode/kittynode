use crate::infra::config::ConfigStore;
use eyre::Result;

pub fn set_server_url(endpoint: String) -> Result<()> {
    let mut config = ConfigStore::load()?;
    config.server_url = endpoint;
    ConfigStore::save(&config)?;
    Ok(())
}
