use std::path::Path;

use crate::docker;

pub fn down(taiko_node_dir: &Path) {
    // Check taiko node is installed first
    if !taiko_node_dir.exists() {
        println!("simple-taiko-node is not installed.");
        return;
    }
    match docker::execute_docker_command(&["compose", "down"], taiko_node_dir) {
        Ok(msg) => {
            println!("{}", msg);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
