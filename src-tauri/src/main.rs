// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controller;

use crate::controller::parse;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
