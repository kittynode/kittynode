use crate::domain::package::{Package, PackageDefinition};
use crate::infra::docker::{find_container, get_docker_instance};
use crate::manifests::ethereum::Ethereum;
use eyre::{Context, Result};
use std::collections::HashMap;

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
