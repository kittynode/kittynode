use crate::{
    domain::package::Package,
    infra::{
        docker::{find_container, get_docker_instance},
        package::get_packages,
    },
};
use eyre::{Context, Result};

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
