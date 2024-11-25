use crate::infra::docker::{
    create_or_recreate_network, find_container, get_docker_instance, pull_and_start_container,
    remove_container,
};
use crate::infra::file::generate_jwt_secret;
use crate::packages::ethereum::Ethereum;
use bollard::models::PortBinding;
use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::{fmt, fs};
use tracing::info;

pub(crate) trait PackageDefinition {
    const NAME: &'static str;
    fn get_package() -> Result<Package>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Package {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) network_name: String,
    pub(crate) containers: Vec<Container>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Container {
    pub(crate) name: String,
    pub(crate) image: String,
    pub(crate) cmd: Vec<String>,
    pub(crate) port_bindings: HashMap<String, Vec<PortBinding>>,
    pub(crate) volume_bindings: Vec<Binding>,
    pub(crate) file_bindings: Vec<Binding>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Binding {
    pub(crate) source: String,
    pub(crate) destination: String,
    pub(crate) options: Option<String>,
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for container in &self.containers {
            writeln!(f, "Container Image: {}", container.image)?;
        }
        Ok(())
    }
}

pub fn get_packages() -> Result<HashMap<String, Package>> {
    let mut packages = HashMap::new();
    packages.insert(Ethereum::NAME.to_string(), Ethereum::get_package()?);
    Ok(packages)
}

pub fn get_package(name: &str) -> Result<Package> {
    let packages = get_packages()?;
    packages
        .get(name)
        .cloned()
        .ok_or_else(|| eyre::eyre!("Package not found"))
}

pub async fn get_installed_packages() -> Result<Vec<Package>> {
    let docker = get_docker_instance()?;
    let packages = get_packages().wrap_err("Failed to retrieve packages")?;

    let mut installed = Vec::new();

    // For each package, check if all containers exist
    for package in packages.values() {
        let mut all_containers_exist = true;

        for container in &package.containers {
            if find_container(&docker, &container.name).await?.is_empty() {
                all_containers_exist = false;
                break;
            }
        }

        // If they do, add the package to the list
        if all_containers_exist {
            installed.push(package.clone());
        }
    }

    Ok(installed)
}

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

pub async fn delete_package(package_name: &str, include_images: bool) -> Result<()> {
    let docker = get_docker_instance()?;
    let packages = get_packages().wrap_err("Failed to retrieve packages")?;
    let package = packages
        .get(package_name)
        .ok_or_else(|| eyre::eyre!("Package '{}' not found", package_name))?;

    let mut image_names = Vec::new();
    let mut file_paths = HashSet::new();
    let mut directory_paths = HashSet::new();
    let mut volume_names = Vec::new();

    for container in &package.containers {
        // Collect the container's image name
        image_names.push(&container.image);

        // Collect the container's volume bindings
        for binding in &container.volume_bindings {
            volume_names.push(&binding.source);
        }

        // Collect the container's file or directory paths
        for binding in &container.file_bindings {
            let local_path = &binding.source;
            // Check if the path exists and ignore the error if it doesn't
            if let Ok(metadata) = fs::metadata(local_path) {
                if metadata.is_dir() {
                    directory_paths.insert(local_path);
                } else {
                    file_paths.insert(local_path);
                }
            } else {
                info!("Path '{}' not found, skipping.", local_path);
            }
        }

        // Remove the container
        remove_container(&docker, &container.name)
            .await
            .wrap_err_with(|| format!("Failed to remove container '{}'", container.name))?;
        info!("Container '{}' removed successfully.", container.name);
    }

    // Remove the container images
    if include_images {
        for image_name in image_names {
            info!("Removing image: {}", image_name);
            docker
                .remove_image(image_name, None, None)
                .await
                .wrap_err_with(|| format!("Failed to remove image '{}'", image_name))?;
            info!("Image '{}' removed successfully.", image_name);
        }
    }

    // Remove the files
    for file_path in file_paths {
        info!("Removing file: {}", file_path);
        fs::remove_file(file_path).wrap_err(format!("Failed to remove file '{}'", file_path))?;
        info!("File '{}' removed successfully.", file_path);
    }

    // Remove the directories after all files have been deleted
    for dir_path in directory_paths {
        info!("Removing directory: {}", dir_path);
        fs::remove_dir_all(dir_path)
            .wrap_err(format!("Failed to remove directory '{}'", dir_path))?;
        info!("Directory '{}' removed successfully.", dir_path);
    }

    // Remove the Docker volumes
    for volume_name in volume_names {
        info!("Removing volume: {}", volume_name);
        docker
            .remove_volume(volume_name, None)
            .await
            .wrap_err_with(|| format!("Failed to remove volume '{}'", volume_name))?;
        info!("Volume '{}' removed successfully.", volume_name);
    }

    // Remove the Docker network
    info!("Removing network: {}", package.network_name);
    docker
        .remove_network(&package.network_name)
        .await
        .wrap_err_with(|| format!("Failed to remove network '{}'", package.network_name))?;
    info!("Network '{}' removed successfully.", package.network_name);

    info!("Package '{}' deleted successfully.", package_name);
    Ok(())
}

pub(crate) fn create_binding_string(binding: &Binding) -> String {
    match &binding.options {
        Some(options) => format!("{}:{}:{}", binding.source, binding.destination, options),
        None => format!("{}:{}", binding.source, binding.destination),
    }
}
