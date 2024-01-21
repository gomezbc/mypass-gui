// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod db;
pub mod models;
pub mod services;
pub mod utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::connect_to_db,
            commands::get_logins,
            commands::check_master_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
