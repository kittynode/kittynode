// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn check_running_nodes() -> u32 {
    0 // Placeholder return value
}

#[tauri::command]
fn install_node() {
    stn_core::install();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_running_nodes, install_node])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
