use crate::domain::package::{Package, PackageDefinition};
use crate::infra::docker::{find_container, get_docker_instance, remove_container};
use crate::manifests::ethereum::Ethereum;
use eyre::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::fs;
use tracing::info;

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
