use std::path::Path;

use crate::{docker, env_manager::EnvManager, network};

pub async fn up(taiko_node_dir: &Path) {
    // Check taiko node is installed first
    if !taiko_node_dir.exists() {
        println!("simple-taiko-node is not installed.");
        return;
    }

    let env_path = taiko_node_dir.join(".env");
    let env_manager = EnvManager::new(&env_path).expect("Failed to initialize EnvManager");
    let l1_endpoint_http = env_manager
        .get("L1_ENDPOINT_HTTP")
        .expect("L1_ENDPOINT_HTTP not set");
    let l1_endpoint_ws = env_manager
        .get("L1_ENDPOINT_WS")
        .expect("L1_ENDPOINT_WS not set");
    let (http_valid, ws_valid) =
        network::validate_endpoints(&l1_endpoint_http, &l1_endpoint_ws).await;
    if !http_valid || !ws_valid {
        println!("L1 endpoints are not healthy. Run `stan config` to set up new endpoints.");
        return;
    }
    match docker::execute_docker_command(&["compose", "up", "-d"], taiko_node_dir) {
        Ok(msg) => {
            println!("{}", msg);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
