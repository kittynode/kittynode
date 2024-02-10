use std::path::Path;

use crate::docker;

pub fn stats(taiko_node_dir: &Path) {
    if !taiko_node_dir.exists() {
        println!("simple-taiko-node is not installed.");
        return;
    }

    match docker::execute_docker_command(&["stats"], taiko_node_dir) {
        Ok(msg) => {
            println!("{}", msg);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
