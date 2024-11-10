use eyre::Result;
use kittynode_core::package::Package;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    LazyLock, RwLock,
};
use tauri_plugin_http::reqwest;
use tracing::info;

/// Global flag to determine if the application is running in remote mode.
/// Initialized to `false` by default.
static IS_REMOTE: AtomicBool = AtomicBool::new(false);

/// Getter for the `IS_REMOTE` flag.
pub fn is_remote() -> bool {
    IS_REMOTE.load(Ordering::SeqCst)
}

/// Setter for the `IS_REMOTE` flag.
pub fn set_remote(value: bool) {
    IS_REMOTE.store(value, Ordering::SeqCst);
}

pub static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());
// pub static SERVER_URL: RwLock<String> = RwLock::new(String::new());
pub static SERVER_URL: LazyLock<RwLock<String>> =
    LazyLock::new(|| RwLock::new(String::from("foobar")));

#[tauri::command]
async fn add_capability(name: String) -> Result<(), String> {
    info!("Adding capability: {}", name);

    if is_remote() {
        let server_url = SERVER_URL
            .read()
            .map_err(|_| "Failed to lock server URL")?
            .clone();
        let url = format!("{}/add_capability/{}", server_url, name);
        let res = HTTP_CLIENT
            .post(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to add capability: {}", res.status()));
        }
        Ok(())
    } else {
        kittynode_core::config::add_capability(&name).map_err(|e| e.to_string())
    }
}

#[tauri::command]
async fn remove_capability(name: String) -> Result<(), String> {
    info!("Removing capability: {}", name);

    if is_remote() {
        let server_url = SERVER_URL
            .read()
            .map_err(|_| "Failed to lock server URL")?
            .clone();
        let url = format!("{}/remove_capability/{}", server_url, name);
        let res = HTTP_CLIENT
            .post(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to remove capability: {}", res.status()));
        }
        Ok(())
    } else {
        kittynode_core::config::remove_capability(&name).map_err(|e| e.to_string())
    }
}

#[tauri::command]
async fn get_capabilities() -> Result<Vec<String>, String> {
    info!("Getting capabilities");

    if is_remote() {
        let server_url = SERVER_URL
            .read()
            .map_err(|_| "Failed to lock server URL")?
            .clone();
        let url = format!("{}/get_capabilities", server_url);
        let res = HTTP_CLIENT
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        // Store status and error text before any potential moves
        let status = res.status();
        let error_text = res.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(format!(
                "Failed to get capabilities: {} - {}",
                status, error_text
            ));
        }
        // Re-issue the request to get JSON, as we cannot reuse `res`
        let res = HTTP_CLIENT
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        res.json::<Vec<String>>().await.map_err(|e| e.to_string())
    } else {
        kittynode_core::config::get_capabilities().map_err(|e| e.to_string())
    }
}

#[tauri::command]
fn get_packages() -> Result<HashMap<String, Package>, String> {
    info!("Getting packages");
    kittynode_core::package::get_packages()
        .map(|packages| {
            packages
                .into_iter()
                .map(|(name, package)| (name.to_string(), package))
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_installed_packages() -> Result<Vec<Package>, String> {
    info!("Getting installed packages");

    if is_remote() {
        let server_url = SERVER_URL
            .read()
            .map_err(|_| "Failed to lock server URL")?
            .clone();
        let url = format!("{}/get_installed_packages", server_url);
        let res = HTTP_CLIENT
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        // Store status and error text before any potential moves
        let status = res.status();
        let error_text = res.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(format!(
                "Failed to get installed packages: {} - {}",
                status, error_text
            ));
        }
        // Re-issue the request to get JSON, as we cannot reuse `res`
        let res = HTTP_CLIENT
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        res.json::<Vec<Package>>().await.map_err(|e| e.to_string())
    } else {
        kittynode_core::package::get_installed_packages()
            .await
            .map_err(|e| e.to_string())
    }
}

#[tauri::command]
async fn is_docker_running() -> bool {
    info!("Checking if Docker is running");
    kittynode_core::docker::is_docker_running().await
}

#[tauri::command]
async fn install_package(name: String) -> Result<(), String> {
    if is_remote() {
        let server_url = SERVER_URL
            .read()
            .map_err(|_| "Failed to lock server URL")?
            .clone();
        let url = format!("{}/install_package/{}", server_url, name);
        let res = HTTP_CLIENT
            .post(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to install package: {}", res.status()));
        }
    } else {
        kittynode_core::package::install_package(&name)
            .await
            .map_err(|e| e.to_string())?;
    }

    info!("Successfully installed package: {}", name);
    Ok(())
}

#[tauri::command]
async fn delete_package(name: String, include_images: bool) -> Result<(), String> {
    if is_remote() {
        let server_url = SERVER_URL
            .read()
            .map_err(|_| "Failed to lock server URL")?
            .clone();
        let url = format!("{}/delete_package/{}", server_url, name);
        let res = HTTP_CLIENT
            .post(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to delete package: {}", res.status()));
        }
    } else {
        kittynode_core::package::delete_package(&name, include_images)
            .await
            .map_err(|e| e.to_string())?;
    }

    info!("Successfully deleted package: {}", name);
    Ok(())
}

#[tauri::command]
async fn delete_kittynode() -> Result<(), String> {
    info!("Deleting .kittynode directory");

    if is_remote() {
        let server_url = SERVER_URL
            .read()
            .map_err(|_| "Failed to lock server URL")?
            .clone();
        let url = format!("{}/delete_kittynode", server_url);
        let res = HTTP_CLIENT
            .post(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to delete Kittynode: {}", res.status()));
        }
        Ok(())
    } else {
        kittynode_core::kittynode::delete_kittynode().map_err(|e| e.to_string())
    }
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
async fn init_kittynode() -> Result<(), String> {
    info!("Initializing Kittynode");

    if is_remote() {
        let server_url = SERVER_URL
            .read()
            .map_err(|_| "Failed to lock server URL")?
            .clone();
        let url = format!("{}/init_kittynode", server_url);
        let res = HTTP_CLIENT
            .post(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(format!("Failed to initialize Kittynode: {}", res.status()));
        }
        Ok(())
    } else {
        kittynode_core::kittynode::init_kittynode().map_err(|e| e.to_string())
    }
}

#[tauri::command]
fn get_is_remote() -> bool {
    is_remote()
}

#[tauri::command]
fn set_is_remote_flag(value: bool) {
    set_remote(value);
}

#[tauri::command]
fn get_server_url() -> Result<String, String> {
    SERVER_URL
        .read()
        .map_err(|_| "Failed to lock server URL".to_string())
        .map(|url| url.clone())
}

#[tauri::command]
fn set_server_url(url: String) -> Result<(), String> {
    let mut server_url = SERVER_URL
        .write()
        .map_err(|_| "Failed to lock server URL".to_string())?;
    *server_url = url;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> eyre::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
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
            add_capability,
            remove_capability,
            get_capabilities,
            get_is_remote,
            set_is_remote_flag,
            get_server_url,
            set_server_url
        ])
        .run(tauri::generate_context!())
        .map_err(|e| eyre::eyre!(e.to_string()))?;

    Ok(())
}
