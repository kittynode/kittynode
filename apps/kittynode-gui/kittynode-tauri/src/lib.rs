use eyre::Result;
use std::collections::HashMap;
use tracing::info;

#[tauri::command]
fn get_packages() -> Result<HashMap<String, kittynode_core::package::Package>, String> {
    info!("Getting packages");
    let packages = kittynode_core::package::get_packages()
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|(name, package)| (name.to_string(), package))
        .collect();
    Ok(packages)
}

#[tauri::command]
async fn get_installed_packages() -> Result<Vec<String>, String> {
    info!("Getting installed packages");
    let installed = kittynode_core::package::get_installed_packages()
        .await
        .map_err(|e| e.to_string())?;
    Ok(installed)
}

#[tauri::command]
async fn install_package(name: String) -> Result<(), String> {
    info!("Installing package: {}", name);
    kittynode_core::package::install_package(&name)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn is_docker_running() -> bool {
    info!("Checking if Docker is running");
    kittynode_core::docker::is_docker_running().await
}

#[tauri::command]
async fn delete_package(name: String, include_images: bool) -> Result<(), String> {
    info!("Deleting package: {}", name);
    kittynode_core::package::delete_package(&name, include_images)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn delete_kittynode() -> Result<(), String> {
    info!("Deleting .kittynode directory");
    kittynode_core::kittynode::delete_kittynode().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn system_info() -> Result<kittynode_core::system_info::SystemInfo, String> {
    info!("Getting system info");
    let system_info = kittynode_core::system_info::get_system_info().map_err(|e| e.to_string())?;
    Ok(system_info)
}

#[tauri::command]
fn is_initialized() -> bool {
    info!("Checking if Kittynode is initialized");
    kittynode_core::kittynode::is_initialized()
}

#[tauri::command]
fn init_kittynode() -> Result<(), String> {
    info!("Initializing Kittynode");
    kittynode_core::kittynode::init_kittynode().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            is_docker_running,
            get_packages,
            install_package,
            get_installed_packages,
            delete_package,
            delete_kittynode,
            system_info,
            is_initialized,
            init_kittynode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
