// -- Structurally non-async commands to register in the invoke handler --

use crate::commands::api::funcs::{add_todo, delete_todo, get_todos};
use crate::models::todo::Todo;

#[tauri::command]
pub fn command_get_todos() -> Result<Vec<Todo>, String> {
    match tauri::async_runtime::block_on(get_todos()) {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn command_add_todo(text: String) -> Result<String, String> {
    match tauri::async_runtime::block_on(add_todo(text)) {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn command_delete_todo(id: usize) -> Result<Option<Todo>, String> {
    match tauri::async_runtime::block_on(delete_todo(id)) {
        Some(result) => Ok(Some(result)),
        None => Ok(None),
    }
}
