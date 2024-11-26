use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub processor: String,
    pub memory: String,
    pub storage: String,
}
