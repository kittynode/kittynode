use crate::file::kittynode_path;
use eyre::Result;
use std::{fs, io::ErrorKind};
use tracing::info;

pub fn init_kittynode() -> Result<()> {
    fs::create_dir_all(kittynode_path()?)?;
    Ok(())
}

pub fn delete_kittynode() -> Result<()> {
    if let Err(e) = fs::remove_dir_all(kittynode_path()?) {
        if e.kind() != ErrorKind::NotFound {
            return Err(e.into());
        }
    }
    info!("Successfully deleted Kittynode.");
    Ok(())
}

pub fn is_initialized() -> bool {
    match kittynode_path() {
        Ok(path) => path.exists(),
        Err(_) => false,
    }
}
