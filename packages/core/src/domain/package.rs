use bollard::models::PortBinding;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub trait PackageDefinition {
    fn name(&self) -> &'static str;
    fn get_package(&self) -> Result<Package>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Package {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) network_name: String,
    pub(crate) containers: Vec<Container>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Container {
    pub(crate) name: String,
    pub(crate) image: String,
    pub(crate) cmd: Vec<String>,
    pub(crate) port_bindings: HashMap<String, Vec<PortBinding>>,
    pub(crate) volume_bindings: Vec<Binding>,
    pub(crate) file_bindings: Vec<Binding>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Binding {
    pub(crate) source: String,
    pub(crate) destination: String,
    pub(crate) options: Option<String>,
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for container in &self.containers {
            writeln!(f, "Container Image: {}", container.image)?;
        }
        Ok(())
    }
}
