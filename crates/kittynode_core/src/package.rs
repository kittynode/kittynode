use crate::file;
use bollard::secret::PortBinding;
use eyre::{Context, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use tracing::info;

#[derive(Serialize)]
pub struct Package {
    description: &'static str,
    containers: Vec<Container>,
}

#[derive(Serialize)]
pub struct Container {
    image: &'static str,
    port_bindings: HashMap<&'static str, Vec<PortBinding>>,
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
    let packages = HashMap::from([(
        "Reth + Lighthouse (Holesky)",
        Package {
            description: "This package installs a Reth execution client and a Lighthouse consensus client on the Holesky network with Docker.",
            containers: vec![
                Container {
                    image: "ghcr.io/paradigmxyz/reth",
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
                },
                Container {
                    image: "sigp/lighthouse",
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
                },
            ],
        },
    )]);
    Ok(packages)
}

pub fn install_package(name: &str) -> Result<()> {
    let packages = get_packages().wrap_err("Failed to retrieve packages")?;
    let package = packages
        .get(name)
        .ok_or_else(|| eyre::eyre!("Package '{}' not found", name))?;
    info!("Installing package: {}", package);

    // Generate shared JWT secret
    file::generate_jwt_secret().wrap_err("Failed to generate JWT secret")?;
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
