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
}
