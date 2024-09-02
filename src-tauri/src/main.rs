#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod algorithms;
mod commands;
mod functions;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::check_config,
            commands::generate_token,
            commands::crypt_string,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
