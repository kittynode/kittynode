use eyre::{Context, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use tracing::info;

#[derive(Serialize)]
pub struct Package {
    version: &'static str,
    containers: Vec<Container>,
}

#[derive(Serialize)]
pub struct Container {
    image: &'static str,
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Version: {}", self.version)?;
        for container in &self.containers {
            writeln!(f, "Container Image: {}", container.image)?;
        }
        Ok(())
    }
}

pub fn get_packages() -> Result<HashMap<&'static str, Package>> {
    let packages = HashMap::from([
        (
            "Reth",
            Package {
                version: "0.1.0",
                containers: vec![Container {
                    image: "ghcr.io/paradigmxyz/reth",
                }],
            },
        ),
        (
            "Lighthouse",
            Package {
                version: "0.1.0",
                containers: vec![Container {
                    image: "sigp/lighthouse",
                }],
            },
        ),
    ]);
    Ok(packages)
}

pub fn install_package(name: &str) -> Result<()> {
    let packages = get_packages().wrap_err("Failed to retrieve packages")?;
    let package = packages
        .get(name)
        .ok_or_else(|| eyre::eyre!("Package '{}' not found", name))?;
    info!("Installing package: {}", package);
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
