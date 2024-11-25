use crate::config::Config;
use eyre::Result;

/// Initializes Kittynode with the default config
pub fn init_kittynode() -> Result<()> {
    let config = Config::default();
    config.save()?;
    Ok(())
}
