use crate::infra::{
    docker::{create_or_recreate_network, get_docker_instance, pull_and_start_container},
    file::generate_jwt_secret,
    package::get_packages,
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

    let network_name = &package.network_name;
    create_or_recreate_network(&docker, network_name).await?;

    for container in &package.containers {
        pull_and_start_container(&docker, container, network_name).await?;
    }

    info!("Package '{}' installed successfully.", name);
    Ok(())
}
