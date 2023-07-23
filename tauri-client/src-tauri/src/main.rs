// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands {
    pub mod api {
        pub mod commands;
        pub mod funcs;
    }
}

mod models {
    pub mod todo;
}

use commands::api::commands::{command_add_todo, command_delete_todo, command_get_todos};

fn main() {
    dotenv::dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command_get_todos,
            command_add_todo,
            command_delete_todo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
