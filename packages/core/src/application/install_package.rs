use crate::{
    infra::{
        docker::{create_or_recreate_network, get_docker_instance, pull_and_start_container},
        file::{generate_jwt_secret, kittynode_path},
        package::get_packages,
        package_config::PackageConfigStore,
    },
    manifests::ethereum::Ethereum,
};
use eyre::{Context, Result};
use tracing::info;

pub async fn install_package(name: &str) -> Result<()> {
    let docker = get_docker_instance()?;

    generate_jwt_secret().wrap_err("Failed to generate JWT secret")?;

    let packages = get_packages().wrap_err("Failed to retrieve packages")?;
    let package = packages
        .get(name)
        .ok_or_else(|| eyre::eyre!("Package '{}' not found", name))?;

    // Load package config or use default
    let config = PackageConfigStore::load(name)?;
    let network_value = "holesky".to_string();
    let network = config.values.get("network").unwrap_or(&network_value);

    // Get containers with the configured network
    let containers = match name {
        "Ethereum" => Ethereum::get_containers(&kittynode_path()?.join("jwt.hex"), network)?,
        _ => package.containers.clone(),
    };

    let network_name = &package.network_name;
    create_or_recreate_network(&docker, network_name).await?;

    for container in &containers {
        pull_and_start_container(&docker, container, network_name).await?;
    }

    info!("Package '{}' installed successfully.", name);
    Ok(())
}
