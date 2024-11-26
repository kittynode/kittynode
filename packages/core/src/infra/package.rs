use crate::domain::package::{Package, PackageDefinition};
use crate::manifests::ethereum::Ethereum;
use eyre::Result;
use std::collections::HashMap;

pub fn get_packages() -> Result<HashMap<String, Package>> {
    let mut packages = HashMap::new();
    packages.insert(Ethereum::NAME.to_string(), Ethereum::get_package()?);
    Ok(packages)
}
