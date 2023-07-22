// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands {
    pub mod api;
}
pub mod models {
    pub mod todo;
}

use commands::api::{add_todo, delete_todo, get_todos};

// Find a way to create tauri commands with satisfication of async functions in the program.
// Obstackles occur within the Tauri app because of awkwards with asyncs.

fn main() {
    dotenv::dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![/* ...commands... */])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
