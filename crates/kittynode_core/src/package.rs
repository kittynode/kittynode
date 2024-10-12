use crate::docker::{create_or_recreate_network, get_docker_instance};
use crate::file::{generate_jwt_secret, kittynode_path};
use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::image::CreateImageOptions;
use bollard::network::ConnectNetworkOptions;
use bollard::secret::{HostConfig, PortBinding};
use bollard::Docker;
use eyre::{Context, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use tokio_stream::StreamExt;
use tracing::{error, info};

#[derive(Serialize)]
pub struct Package {
    description: &'static str,
    network_name: &'static str,
    containers: Vec<Container>,
}

#[derive(Serialize)]
pub struct Container {
    name: &'static str,
    image: &'static str,
    cmd: Vec<&'static str>,
    port_bindings: HashMap<&'static str, Vec<PortBinding>>,
    volume_bindings: Vec<String>,
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
                        "rethdata:/root/.local/share/reth/mainnet".to_string(),
                        format!("{}:/root/.local/share/reth/mainnet/jwt.hex:ro", jwt_path.display()),
                    ],
                },
                Container {
                    name: "lighthouse-node",
                    image: "sigp/lighthouse",
                    cmd: vec![
                        "lighthouse",
                        "--network",
                        "mainnet",
                        "beacon",
                        "--http",
                        "--http-address",
                        "0.0.0.0",
                        "--checkpoint-sync-url",
                        "https://mainnet.checkpoint.sigp.io",
                        "--execution-jwt",
                        "/root/.lighthouse/mainnet/jwt.hex",
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
                        format!("{}:/root/.lighthouse/mainnet/jwt.hex:ro", jwt_path.display()),
                    ],
                },
            ],
        },
    )]);
    Ok(packages)
}

pub async fn install_package(name: &str) -> Result<()> {
    // Initialize Docker instance (note: should inject this and/or handle in docker.rs)
    let docker = get_docker_instance()?;

    info!("Installing package: {}", name);

    // Generate JWT secret
    generate_jwt_secret().wrap_err("Failed to generate JWT secret")?;

    let packages = get_packages().wrap_err("Failed to retrieve packages")?;
    let package = packages
        .get(name)
        .ok_or_else(|| eyre::eyre!("Package '{}' not found", name))?;

    if package_exists(&docker, package).await? {
        return Err(eyre::eyre!("Package '{}' already exists", name));
    }

    create_or_recreate_network(&docker, package.network_name).await?;

    for container in &package.containers {
        pull_and_start_container(&docker, container, package.network_name).await?;
    }

    Ok(())
}

async fn package_exists(docker: &Docker, package: &Package) -> Result<bool> {
    let containers = docker.list_containers::<String>(None).await?;

    for container in &package.containers {
        if containers.iter().any(|c| {
            c.names
                .as_ref() // Get reference to the Option<Vec<String>>
                .map_or(false, |names| {
                    names.contains(&format!("/{}", container.name))
                }) // Check if the name exists
        }) {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn pull_and_start_container(
    docker: &Docker,
    container: &Container,
    network_name: &str,
) -> Result<()> {
    // Pull the container image
    let options = Some(CreateImageOptions {
        from_image: container.image,
        tag: "latest",
        ..Default::default()
    });

    let mut stream = docker.create_image(options, None, None);
    while let Some(item) = stream.next().await {
        match item {
            Ok(info) => info!("Pulling image info: {:?}", info),
            Err(e) => error!("Error pulling image: {:?}", e),
        }
    }

    // Convert port bindings to the correct type
    let port_bindings: HashMap<String, Option<Vec<PortBinding>>> = container
        .port_bindings
        .iter()
        .map(|(k, v)| (k.to_string(), Some(v.clone())))
        .collect();

    let host_config = HostConfig {
        binds: Some(container.volume_bindings.clone()),
        port_bindings: Some(port_bindings), // Use the converted bindings
        ..Default::default()
    };

    let config = Config {
        image: Some(container.image),
        cmd: Some(container.cmd.clone()),
        host_config: Some(host_config),
        ..Default::default()
    };

    // Create the container
    let created_container = docker
        .create_container(
            Some(CreateContainerOptions {
                name: container.name,
                ..Default::default()
            }),
            config,
        )
        .await?;

    // Start the container
    docker
        .start_container(&created_container.id, None::<StartContainerOptions<String>>)
        .await?;

    info!("Container {} started successfully.", container.name);

    // Connect the container to the network
    docker
        .connect_network(
            network_name,
            ConnectNetworkOptions {
                container: container.name,
                endpoint_config: Default::default(),
            },
        )
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_prints_all_packages() {
        let packages = get_packages().expect("Failed to get packages");
        for (name, package) in packages {
            println!("Package: {}\n{}", name, package);
        }
    }
}
