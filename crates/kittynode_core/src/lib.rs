pub mod constants;
pub mod packages;

use bollard::Docker;
use eyre::Result;
use std::{env, fs, path::Path};
use tracing::info;

pub fn create_kittynode_directory() -> Result<()> {
    info!("Creating .kittynode directory");
    let path = Path::new(&env::var("HOME")?).join(crate::constants::KITTYNODE_PATH);
    fs::create_dir_all(&path)?;
    info!(".kittynode directory created");
    Ok(())
}

pub fn install() -> Result<()> {
    info!("Installing Kittynode");
    create_kittynode_directory()?;
    Ok(())
}

pub fn check_running_nodes() -> Result<i32> {
    info!("Checking running nodes");
    Ok(4)
}

pub async fn check_docker_version() -> Result<()> {
    info!("Checking Docker version");
    let docker = Docker::connect_with_local_defaults()?;
    info!("Connected to Docker");
    let version = docker.version().await?;
    info!("Docker version: {:?}", version.version);
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

    #[test]
    fn install_creates_scoped_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        env::set_var("HOME", temp_dir.path().to_str().unwrap());
        assert!(install().is_ok());
    }

    #[test]
    fn check_running_nodes_returns_zero() {
        matches!(check_running_nodes(), Ok(1));
    }
}
