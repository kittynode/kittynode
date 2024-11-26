use crate::domain::package::Package;
use crate::infra;
use eyre::Result;
use std::collections::HashMap;

pub fn get_packages() -> Result<HashMap<String, Package>> {
    infra::package::get_packages()
}
