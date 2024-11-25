use crate::{config::Config, infra::file::kittynode_path};
use eyre::Result;
use std::{fs, io::ErrorKind};
use tracing::info;

/// Initializes the Kittynode by creating the directory and default config file.
pub fn init_kittynode() -> Result<()> {
    let config = Config::default();
    config.save()?;
    Ok(())
}

/// Deletes the Kittynode directory and its contents.
pub fn delete_kittynode() -> Result<()> {
    if let Err(e) = fs::remove_dir_all(kittynode_path()?) {
        if e.kind() != ErrorKind::NotFound {
            return Err(e.into());
        }
    }
    info!("Successfully deleted Kittynode.");
    Ok(())
}
