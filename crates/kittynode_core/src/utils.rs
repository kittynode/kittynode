use crate::constants::KITTYNODE_PATH;
use std::path::{Path, PathBuf};
use std::{env, io};

/// Helper function that returns the path to the kittynode config directory.
pub fn get_kittynode_directory() -> Result<PathBuf, io::Error> {
    let home_dir = env::var("HOME")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Failed to get home directory"))?;
    Ok(Path::new(&home_dir).join(KITTYNODE_PATH))
}
