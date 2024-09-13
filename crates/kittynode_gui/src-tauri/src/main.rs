// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eyre::Result;

#[tauri::command]
fn check_running_nodes() -> Result<i32, String> {
    kittynode_core::check_running_nodes().map_err(|e| e.to_string())?;
    Ok(0)
}

#[tauri::command]
fn install_node() -> Result<(), String> {
    kittynode_core::install().map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_running_nodes, install_node])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
