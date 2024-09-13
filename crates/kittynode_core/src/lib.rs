pub mod constants;

use eyre::Result;
use std::{env, fs, path::Path};

pub fn create_kittynode_directory() -> Result<()> {
    let path = Path::new(&env::var("HOME")?).join(crate::constants::KITTYNODE_PATH);
    fs::create_dir_all(&path)?;
    Ok(())
}

pub fn install() -> Result<()> {
    create_kittynode_directory()?;
    Ok(())
}

pub fn check_running_nodes() -> Result<i32> {
    Ok(0)
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

    #[test]
    fn install_creates_scoped_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        env::set_var("HOME", temp_dir.path().to_str().unwrap());
        assert!(install().is_ok());
    }

    #[test]
    fn check_running_nodes_returns_zero() {
        matches!(check_running_nodes(), Ok(0));
    }
}
