use eyre::Result;
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct Package {
    package: PackageInfo,
    containers: Vec<Container>,
}

#[derive(Serialize)]
pub struct PackageInfo {
    name: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
pub struct Container {
    image: &'static str,
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Package: {}, Version: {}",
            self.package.name, self.package.version
        )?;
        for container in &self.containers {
            writeln!(f, "Container Image: {}", container.image)?;
        }
        Ok(())
    }
}

pub fn get_packages() -> Result<Vec<Package>> {
    let packages: Vec<Package> = vec![
        Package {
            package: PackageInfo {
                name: "Reth",
                version: "0.1.0",
            },
            containers: vec![Container {
                image: "ghcr.io/paradigmxyz/reth",
            }],
        },
        Package {
            package: PackageInfo {
                name: "Lighthouse",
                version: "0.1.0",
            },
            containers: vec![Container {
                image: "sigp/lighthouse",
            }],
        },
    ];
    Ok(packages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_prints_all_packages() {
        let packages = get_packages().expect("Failed to get packages");
        for package in packages {
            println!("{}", package);
        }
    }
}
