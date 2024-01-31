use std::path::Path;

use crate::{docker, utils};

pub fn stats(taiko_node_dir: &Path) {
    if !taiko_node_dir.exists() {
        utils::stn_log("simple-taiko-node is not installed.");
        return;
    }

    match docker::execute_docker_command(&["stats"], taiko_node_dir) {
        Ok(msg) => {
            utils::stn_log(&msg);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
