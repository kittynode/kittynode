use eyre::{Context, Result};
use std::{env, fs, path::Path};
use tracing::info;

pub fn create_kittynode_directory() -> Result<()> {
    info!("Creating .kittynode directory");
    let home_dir = env::var("HOME").wrap_err("Failed to read HOME environment variable")?;
    let path = Path::new(&home_dir).join(crate::constants::KITTYNODE_PATH);
    fs::create_dir_all(&path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_the_kittynode_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        env::set_var("HOME", temp_dir.path().to_str().unwrap());
        let result = create_kittynode_directory();
        assert!(result.is_ok());
    }
}
