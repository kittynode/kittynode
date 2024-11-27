use crate::domain::package::{Package, PackageDefinition};
use crate::manifests::ethereum::Ethereum;
use eyre::Result;
use std::collections::HashMap;

/// Retrieves a HashMap of all available packages.
pub fn get_packages() -> Result<HashMap<String, Package>> {
    let mut packages = HashMap::new();
    insert_package(&mut packages, &Ethereum)?;
    Ok(packages)
}

/// Inserts a package into the HashMap.
fn insert_package(
    packages: &mut HashMap<String, Package>,
    package_def: &impl PackageDefinition,
) -> Result<()> {
    packages.insert(package_def.name().to_string(), package_def.get_package()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifests::ethereum::Ethereum;

    #[test]
    fn inserts_package_successfully() {
        let mut packages = HashMap::new();
        let ethereum = Ethereum;
        insert_package(&mut packages, &ethereum).unwrap();
        assert!(packages.contains_key(ethereum.name()));
    }

    #[test]
    fn retrieves_all_packages() {
        let packages = get_packages().unwrap();
        let ethereum = Ethereum;
        assert!(packages.contains_key(ethereum.name()));
    }
}
