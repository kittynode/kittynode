use eyre::{Result, WrapErr};
use tracing::info;

#[tauri::command]
fn check_running_nodes() -> Result<i32, String> {
    info!("Checking running nodes");
    let num_nodes = kittynode_core::check_running_nodes()
        .wrap_err("Error checking running nodes")
        .map_err(|e| e.to_string())?;
    Ok(num_nodes)
}

#[tauri::command]
fn install_node() -> Result<(), String> {
    info!("Installing node");
    kittynode_core::install()
        .wrap_err("Error installing node")
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn check_docker_version() -> Result<(), String> {
    info!("Checking docker version");
    kittynode_core::check_docker_version()
        .await
        .wrap_err("Error checking docker version")
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            check_running_nodes,
            install_node,
            check_docker_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
