use eyre::Result;
use kittynode_core::package::Package;
use std::collections::HashMap;
use tracing::info;

#[cfg(mobile)]
use once_cell::sync::OnceCell;
#[cfg(mobile)]
use tauri_plugin_http::reqwest;

#[cfg(mobile)]
pub static HTTP_CLIENT: OnceCell<reqwest::Client> = OnceCell::new();
#[cfg(mobile)]
pub static SERVER_URL: OnceCell<String> = OnceCell::new();

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
async fn get_installed_packages() -> Result<Vec<Package>, String> {
    info!("Getting installed packages");
    #[cfg(not(mobile))]
    {
        let installed = kittynode_core::package::get_installed_packages()
            .await
            .map_err(|e| e.to_string())?;
        Ok(installed)
    }
    #[cfg(mobile)]
    {
        let client = HTTP_CLIENT.get_or_init(reqwest::Client::new);
        let server_url = SERVER_URL.get().ok_or("Server URL not set")?;
        let url = format!("{}/get_installed_packages", server_url);
        let res = client.get(&url).send().await.map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!(
                "Failed to get installed packages: {}",
                res.status()
            ));
        }
        Ok(res.json::<Vec<String>>().await?)
    }
}

#[tauri::command]
async fn is_docker_running() -> bool {
    info!("Checking if Docker is running");
    kittynode_core::docker::is_docker_running().await
}

#[tauri::command]
async fn install_package(name: String) -> Result<(), String> {
    #[cfg(not(mobile))]
    kittynode_core::package::install_package(&name)
        .await
        .map_err(|e| e.to_string())?;

    #[cfg(mobile)]
    {
        let client = HTTP_CLIENT.get_or_init(reqwest::Client::new);
        let server_url = SERVER_URL.get().ok_or("Server URL not set")?;
        let url = format!("{}/install_package/{}", server_url, name);
        let res = client.post(&url).send().await.map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to install package: {}", res.status()));
        }
    }

    info!("Successfully installed package: {}", name);
    Ok(())
}

#[tauri::command]
async fn delete_package(name: String, include_images: bool) -> Result<(), String> {
    #[cfg(not(mobile))]
    kittynode_core::package::delete_package(&name, include_images)
        .await
        .map_err(|e| e.to_string())?;

    #[cfg(mobile)]
    {
        let client = HTTP_CLIENT.get_or_init(reqwest::Client::new);
        let server_url = SERVER_URL.get().ok_or("Server URL not set")?;
        let url = format!("{}/delete_package/{}", server_url, name);
        let res = client.post(&url).send().await.map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to delete package: {}", res.status()));
        }
    }

    info!("Successfully deleted package: {}", name);
    Ok(())
}

#[tauri::command]
fn delete_kittynode() -> Result<(), String> {
    info!("Deleting .kittynode directory");
    kittynode_core::kittynode::delete_kittynode().map_err(|e| e.to_string())
}

#[tauri::command]
fn system_info() -> Result<kittynode_core::system_info::SystemInfo, String> {
    info!("Getting system info");
    kittynode_core::system_info::get_system_info().map_err(|e| e.to_string())
}

#[tauri::command]
fn is_initialized() -> bool {
    info!("Checking if Kittynode is initialized");
    kittynode_core::kittynode::is_initialized()
}

#[tauri::command]
fn init_kittynode() -> Result<(), String> {
    info!("Initializing Kittynode");
    kittynode_core::kittynode::init_kittynode().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_| {
            #[cfg(mobile)]
            SERVER_URL.set("http://merlin:3000".to_string())?;
            Ok(()) // do nothing if not mobile
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_packages,
            get_installed_packages,
            is_docker_running,
            install_package,
            delete_package,
            delete_kittynode,
            system_info,
            is_initialized,
            init_kittynode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
