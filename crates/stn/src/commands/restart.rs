use std::path::Path;

use crate::commands::{down, up};

pub async fn restart(taiko_node_dir: &Path) {
    down(taiko_node_dir);
    up(taiko_node_dir).await;
}
