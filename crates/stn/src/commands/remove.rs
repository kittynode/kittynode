use std::{fs, path::Path};

use crate::docker;

pub fn remove(taiko_node_dir: &Path) {
    // Check if taiko node is installed first
    if !taiko_node_dir.exists() {
        println!("simple-taiko-node is not installed.");
        return;
    }
    match docker::execute_docker_command(&["compose", "down", "-v"], taiko_node_dir) {
        Ok(msg) => {
            println!("{}", msg);
        }
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }

    println!("simple-taiko-node volumes deleted from system");
    match fs::remove_dir_all(taiko_node_dir) {
        Ok(_) => {
            println!("simple-taiko-node directory deleted from system");
        }
        Err(e) => {
            eprintln!("Failed to remove simple-taiko-node directory: {}", e);
        }
    }
}
