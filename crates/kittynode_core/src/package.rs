use eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Package {
    package: PackageInfo,
    containers: Vec<Container>,
}

#[derive(Deserialize, Serialize)]
pub struct PackageInfo {
    name: String,
    version: String,
}

#[derive(Deserialize, Serialize)]
pub struct Container {
    image: String,
}

pub fn get_packages() -> Result<HashMap<String, Package>> {
    let mut packages = HashMap::new();

    // Add packages here
    let reth = Package {
        package: PackageInfo {
            name: "Reth".to_string(),
            version: "0.1.0".to_string(),
        },
        containers: vec![Container {
            image: "ghcr.io/paradigmxyz/reth".to_string(),
        }],
    };

    let lighthouse = Package {
        package: PackageInfo {
            name: "Lighthouse".to_string(),
            version: "0.1.0".to_string(),
        },
        containers: vec![Container {
            image: "sigp/lighthouse".to_string(),
        }],
    };

    packages.insert(reth.package.name.clone(), reth);
    packages.insert(lighthouse.package.name.clone(), lighthouse);
    Ok(packages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_prints_all_packages() {
        let registry = get_packages().expect("Failed to get registry");
        for package in registry.values() {
            println!(
                "Package: {}, Version: {}",
                package.package.name, package.package.version
            );
            for container in package.containers.iter() {
                println!("Container Image: {}", container.image);
            }
        }
    }
}
