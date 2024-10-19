use crate::docker::{
    create_or_recreate_network, find_container, get_docker_instance, pull_and_start_container,
    remove_container,
};
use crate::file::{generate_jwt_secret, kittynode_path};
use bollard::secret::PortBinding;
use eyre::{Context, Result};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::{fmt, fs};
use tracing::info;

#[derive(Serialize)]
pub struct Package {
    pub(crate) description: &'static str,
    pub(crate) network_name: &'static str,
    pub(crate) containers: Vec<Container>,
}

#[derive(Serialize)]
pub struct Container {
    pub(crate) name: &'static str,
    pub(crate) image: &'static str,
    pub(crate) cmd: Vec<&'static str>,
    pub(crate) port_bindings: HashMap<&'static str, Vec<PortBinding>>,
    pub(crate) volume_bindings: Vec<Binding>,
    pub(crate) file_bindings: Vec<Binding>,
}

#[derive(Serialize)]
pub struct Binding {
    source: String,
    destination: String,
    options: Option<String>,
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for container in &self.containers {
            writeln!(f, "Container Image: {}", container.image)?;
        }
        Ok(())
    }
}

pub(crate) fn create_binding_string(binding: &Binding) -> String {
    match &binding.options {
        Some(options) => format!("{}:{}:{}", binding.source, binding.destination, options),
        None => format!("{}:{}", binding.source, binding.destination),
    }
}

pub fn get_packages() -> Result<HashMap<&'static str, Package>> {
    let kittynode_path = kittynode_path()?;
    let jwt_path = kittynode_path.join("jwt.hex");

    let packages = HashMap::from([(
        "Reth + Lighthouse (Holesky)",
        Package {
            description: "This package installs a Reth execution client and a Lighthouse consensus client on the Holesky network with Docker.",
            network_name: "reth-lighthouse-holesky-network",
            containers: vec![
                Container {
                    name: "reth-node",
                    image: "ghcr.io/paradigmxyz/reth",
                    cmd: vec![
                        "node",
                        "--chain",
                        "holesky",
                        "--metrics",
                        "0.0.0.0:9001",
                        "--authrpc.addr",
                        "0.0.0.0",
                        "--authrpc.port",
                        "8551",
                    ],
                    port_bindings: HashMap::from([
                        (
                            "9001/tcp",
                            vec![PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("9001".to_string()),
                            }],
                        ),
                        (
                            "30303/tcp",
                            vec![PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("30303".to_string()),
                            }],
                        ),
                        (
                            "30303/udp",
                            vec![PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("30303".to_string()),
                            }],
                        ),
                    ]),
                    volume_bindings: vec![
                      Binding {
                        source: "rethdata".to_string(),
                        destination: "/root/.local/share/reth/holesky".to_string(),
                        options: None,
                      }
                    ],
                    file_bindings: vec![
                      Binding {
                        source: jwt_path.display().to_string(),
                        destination: "/root/.local/share/reth/holesky/jwt.hex".to_string(),
                        options: Some("ro".to_string()),
                        }
                    ],
                },
                Container {
                    name: "lighthouse-node",
                    image: "sigp/lighthouse",
                    cmd: vec![
                        "lighthouse",
                        "--network",
                        "holesky",
                        "beacon",
                        "--http",
                        "--http-address",
                        "0.0.0.0",
                        "--checkpoint-sync-url",
                        "https://checkpoint-sync.holesky.ethpandaops.io",
                        "--execution-jwt",
                        "/root/.lighthouse/holesky/jwt.hex",
                        "--execution-endpoint",
                        "http://reth-node:8551",
                    ],
                    port_bindings: HashMap::from([
                        (
                            "9000/tcp",
                            vec![PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("9000".to_string()),
                            }],
                        ),
                        (
                            "9000/udp",
                            vec![PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("9000".to_string()),
                            }],
                        ),
                        (
                            "9001/udp",
                            vec![PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("9001".to_string()),
                            }],
                        ),
                        (
                            "5052/tcp",
                            vec![PortBinding {
                                host_ip: Some("127.0.0.1".to_string()),
                                host_port: Some("5052".to_string()),
                            }],
                        ),
                    ]),
                    volume_bindings: vec![],
                    file_bindings: vec![
                      Binding {
                        source: kittynode_path.display().to_string(),
                        destination: "/root/.lighthouse".to_string(),
                        options: None,
                      },
                      Binding {
                        source: jwt_path.display().to_string(),
                        destination: "/root/.lighthouse/holesky/jwt.hex".to_string(),
                        options: Some("ro".to_string()),
                        }
                    ],
                },
            ],
        },
    )]);
    Ok(packages)
}

pub async fn get_installed_packages() -> Result<Vec<String>> {
    let docker = get_docker_instance()?;
    let packages = get_packages().wrap_err("Failed to retrieve packages")?;

    let mut installed = Vec::new();

    // Check if containers for each package exist
    for (name, package) in packages {
        let mut all_containers_running = true;

        // Ensure all containers for the package are running
        for container in &package.containers {
            if find_container(&docker, container.name).await?.is_none() {
                all_containers_running = false;
                break;
            }
        }

        if all_containers_running {
            installed.push(name.to_string());
        }
    }

    Ok(installed)
}

pub async fn install_package(name: &str) -> Result<()> {
    let docker = get_docker_instance()?;

    info!("Installing package: {}", name);

    generate_jwt_secret().wrap_err("Failed to generate JWT secret")?;

    let packages = get_packages().wrap_err("Failed to retrieve packages")?;
    let package = packages
        .get(name)
        .ok_or_else(|| eyre::eyre!("Package '{}' not found", name))?;

    let network_name = package.network_name;
    create_or_recreate_network(&docker, network_name).await?;

    for container in &package.containers {
        pull_and_start_container(&docker, container, network_name).await?;
    }

    info!("Package '{}' installed successfully.", name);
    Ok(())
}

pub async fn delete_package(package_name: &str) -> Result<()> {
    let docker = get_docker_instance()?;
    let packages = get_packages().wrap_err("Failed to retrieve packages")?;
    let package = packages
        .get(package_name)
        .ok_or_else(|| eyre::eyre!("Package '{}' not found", package_name))?;

    let mut image_names: Vec<String> = Vec::new();
    let mut file_paths: HashSet<String> = HashSet::new();
    let mut directory_paths: HashSet<String> = HashSet::new();
    let mut volume_names: Vec<String> = Vec::new();

    for container in &package.containers {
        // Collect the container's image name
        image_names.push(container.image.to_string());

        // Collect the container's volume bindings
        for binding in &container.volume_bindings {
            volume_names.push(binding.source.to_string());
        }

        // Collect the container's file or directory paths
        for binding in &container.file_bindings {
            let local_path = &binding.source.to_string();
            let metadata = fs::metadata(local_path)
                .wrap_err(format!("Failed to get metadata for '{}'", local_path))?;
            if metadata.is_dir() {
                directory_paths.insert(local_path.to_string());
            } else {
                file_paths.insert(local_path.to_string());
            }
        }

        // Remove the container
        remove_container(&docker, container.name)
            .await
            .wrap_err_with(|| format!("Failed to remove container '{}'", container.name))?;
    }

    // Remove the container images
    for image_name in image_names {
        info!("Removing image: {}", image_name);
        docker
            .remove_image(&image_name, None, None)
            .await
            .wrap_err_with(|| format!("Failed to remove image '{}'", image_name))?;
        info!("Image '{}' removed successfully.", image_name);
    }

    // Remove the files
    for file_path in &file_paths {
        info!("Removing file: {}", file_path);
        fs::remove_file(file_path).wrap_err(format!("Failed to remove file '{}'", file_path))?;
        info!("File '{}' removed successfully.", file_path);
    }

    // Remove the directories after all files have been deleted
    for dir_path in &directory_paths {
        info!("Removing directory: {}", dir_path);
        fs::remove_dir_all(dir_path)
            .wrap_err(format!("Failed to remove directory '{}'", dir_path))?;
        info!("Directory '{}' removed successfully.", dir_path);
    }

    // Remove the Docker volumes
    for volume_name in volume_names {
        info!("Removing volume: {}", volume_name);
        docker
            .remove_volume(&volume_name, None)
            .await
            .wrap_err_with(|| format!("Failed to remove volume '{}'", volume_name))?;
        info!("Volume '{}' removed successfully.", volume_name);
    }

    // Remove the Docker network
    info!("Removing network: {}", package.network_name);
    docker
        .remove_network(package.network_name)
        .await
        .wrap_err_with(|| format!("Failed to remove network '{}'", package.network_name))?;
    info!("Network '{}' removed successfully.", package.network_name);

    info!("Package '{}' deleted successfully.", package_name);
    Ok(())
}
