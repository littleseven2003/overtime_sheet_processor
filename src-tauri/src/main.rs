// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod excel;
mod models;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::file::open_file_dialog,
            commands::file::save_file_dialog,
            commands::file::read_file_meta,
            commands::process::process_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
