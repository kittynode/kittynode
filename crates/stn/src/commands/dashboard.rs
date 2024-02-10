use std::path::Path;

use crate::env_manager::EnvManager;

pub fn dashboard(taiko_node_dir: &Path) {
    let env_path = taiko_node_dir.join(".env");
    let env_manager = EnvManager::new(&env_path).expect("Failed to initialize EnvManager");
    let grafana_port = env_manager
        .get("PORT_GRAFANA")
        .expect("PORT_GRAFANA not set");
    let grafana_url = format!("http://localhost:{}", grafana_port);
    match webbrowser::open(&grafana_url) {
        Ok(_) => {
            println!("Opened Grafana dashboard at {}", grafana_url);
        }
        Err(e) => {
            eprintln!("Failed to open Grafana dashboard: {}", e);
        }
    }
}
