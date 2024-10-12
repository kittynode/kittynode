use crate::docker::{
    create_or_recreate_network, find_container, get_docker_instance, pull_and_start_container,
    remove_container,
};
use crate::file::{generate_jwt_secret, kittynode_path};
use bollard::secret::PortBinding;
use eyre::{Context, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use tracing::info;

#[derive(Serialize)]
pub struct Package {
    pub description: &'static str,
    pub network_name: &'static str,
    pub containers: Vec<Container>,
}

#[derive(Serialize)]
pub struct Container {
    pub name: &'static str,
    pub image: &'static str,
    pub cmd: Vec<&'static str>,
    pub port_bindings: HashMap<&'static str, Vec<PortBinding>>,
    pub volume_bindings: Vec<String>,
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for container in &self.containers {
            writeln!(f, "Container Image: {}", container.image)?;
        }
        Ok(())
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
                        "rethdata:/root/.local/share/reth/holesky".to_string(),
                        format!("{}:/root/.local/share/reth/holesky/jwt.hex:ro", jwt_path.display()),
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
                    volume_bindings: vec![
                        format!("{}/.lighthouse:/root/.lighthouse", kittynode_path.display()),
                        format!("{}:/root/.lighthouse/holesky/jwt.hex:ro", jwt_path.display()),
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

pub async fn delete_package(name: &str) -> Result<()> {
    let docker = get_docker_instance()?;
    let packages = get_packages().wrap_err("Failed to retrieve packages")?;
    let package = packages
        .get(name)
        .ok_or_else(|| eyre::eyre!("Package '{}' not found", name))?;

    let container_names: Vec<&str> = package.containers.iter().map(|c| c.name).collect();

    for container_name in container_names {
        remove_container(&docker, container_name).await?;
    }

    info!("Package '{}' deleted successfully.", name);
    Ok(())
}
