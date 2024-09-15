// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eyre::{Result, WrapErr};
use tracing::info;

#[tauri::command]
fn check_running_nodes() -> Result<i32, String> {
    info!("Checking running nodes");
    kittynode_core::check_running_nodes()
        .wrap_err("Error checking running nodes")
        .map_err(|e| e.to_string())?;
    Ok(0)
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

fn main() {
    tracing_subscriber::fmt::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_running_nodes,
            install_node,
            check_docker_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
