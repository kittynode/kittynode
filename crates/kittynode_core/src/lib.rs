mod constants;
pub mod package;
use bollard::Docker;
use eyre::{Result, WrapErr};
use std::{env, fs, path::Path};
use tracing::info;

pub fn create_kittynode_directory() -> Result<()> {
    info!("Creating .kittynode directory");
    let home_dir = env::var("HOME").wrap_err("Failed to read HOME environment variable")?;
    let path = Path::new(&home_dir).join(crate::constants::KITTYNODE_PATH);
    fs::create_dir_all(&path)?;
    Ok(())
}

pub fn install() -> Result<()> {
    info!("Installing Kittynode");
    create_kittynode_directory()?;
    Ok(())
}

pub fn check_running_nodes() -> Result<i32> {
    info!("Checking running nodes");
    Ok(0)
}

pub async fn check_docker_version() -> Result<()> {
    info!("Checking Docker version");
    let docker = Docker::connect_with_local_defaults()
        .wrap_err("Failed to connect to Docker with local defaults")?;
    let version = docker
        .version()
        .await
        .wrap_err("Failed to fetch Docker version")?;
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
        assert!(matches!(check_running_nodes(), Ok(0)));
    }
}
