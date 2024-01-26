// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_system;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("Hello, {}!", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            file_system::read_dir,
            file_system::read_file,
            file_system::create_dir,
            file_system::create_file,
            file_system::write_file,
            file_system::update_file,
            file_system::delete_file,
            file_system::del_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
