use eyre::Result;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Package {
    package: PackageInfo,
    containers: Vec<Container>,
}

#[derive(Deserialize)]
pub struct PackageInfo {
    name: String,
    version: String,
}

#[derive(Deserialize)]
pub struct Container {
    image: String,
}

pub fn parse_package(path: &str) -> Result<Package> {
    let package: Package = toml::from_str(&fs::read_to_string(path)?)?;
    Ok(package)
}

pub fn get_packages() -> Result<Vec<Package>> {
    let packages_dir = format!("{}/../../packages", env!("CARGO_MANIFEST_DIR"));
    let packages = fs::read_dir(packages_dir)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let package_toml_path = path.join("package.toml");
            package_toml_path
                .to_str()
                .and_then(|path_str| parse_package(path_str).ok())
        })
        .collect();
    Ok(packages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_reth_package() {
        let path = format!(
            "{}/../../packages/reth/package.toml",
            env!("CARGO_MANIFEST_DIR")
        );
        assert!(parse_package(&path).is_ok());
    }

    #[test]
    fn it_prints_all_packages() {
        let packages = get_packages().expect("Failed to get packages");
        for package in packages {
            println!(
                "Package: {}, Version: {}",
                package.package.name, package.package.version
            );
            for container in package.containers {
                println!("Container Image: {}", container.image);
            }
        }
    }
}
