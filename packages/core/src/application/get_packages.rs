use crate::domain::package::Package;
use eyre::Result;
use std::collections::HashMap;

pub fn get_packages() -> Result<HashMap<String, Package>> {
    crate::infra::package::get_packages()
}
