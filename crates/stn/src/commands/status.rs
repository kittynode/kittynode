use std::path::Path;

use crate::{
    constants,
    env_manager::EnvManager,
    network::{self, get_sync_state},
};

pub async fn status(taiko_node_dir: &Path) {
    // Check taiko node is installed first
    if !taiko_node_dir.exists() {
        println!("simple-taiko-node is not installed.");
        return;
    }

    let env_path = taiko_node_dir.join(".env");
    let env_manager = EnvManager::new(&env_path).expect("Failed to initialize EnvManager");

    let l2_endpoint_port = env_manager
        .get("PORT_L2_EXECUTION_ENGINE_HTTP")
        .expect("PORT_L2_EXECUTION_ENGINE_HTTP not set");
    let l2_endpoint_http = format!("http://localhost:{}", l2_endpoint_port);

    match get_sync_state(&l2_endpoint_http).await {
        Ok(sync_state) => {
            if sync_state.is_syncing {
                println!("Syncing in progress: {:.2}% complete.", sync_state.progress);
            } else {
                // If not syncing, check if it is finished syncing
                let is_fully_synced =
                    network::is_synced(&l2_endpoint_http, constants::KATLA_RPC_URL).await;
                if is_fully_synced {
                    println!("Node is fully synced.");
                } else {
                    println!("Node is not syncing.");
                }
            }
        }
        Err(error) => {
            eprintln!("{} Are you sure your node is running?", error);
        }
    }
}
