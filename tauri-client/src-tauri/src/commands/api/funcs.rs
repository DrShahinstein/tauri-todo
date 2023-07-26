use crate::models::todo::Todo;
use reqwest::Client;
use std::env;

fn api() -> String {
    match env::var("API_BASE_URL") {
        Ok(url) => url,
        Err(_) => {
            let default_url = "http://127.0.0.1:8000/api";
            println!("API_BASE_URL environment variable not set. Using default: {}", default_url);
            default_url.to_string()
        }
    }
}

pub async fn delete_todo(id: usize) -> Option<Todo> {
    tauri::async_runtime::spawn(async move {
        let url = format!("{}/delete_todo/{}", api(), id);
        let client = Client::new();
        let response = client.delete(&url).send().await.ok()?;
        response.json().await.ok()
    })
    .await
    .unwrap()
}

pub async fn add_todo(text: String) -> Result<String, reqwest::Error> {
    tauri::async_runtime::spawn(async move {
        let url = format!("{}/add_todo", api());
        let client = Client::new();
        let response = client.post(&url).body(text).send().await?;
        response.text().await
    })
    .await
    .unwrap()
}

pub async fn get_todos() -> Result<Vec<Todo>, reqwest::Error> {
    tauri::async_runtime::spawn(async move {
        let url = format!("{}/get_todos", api());
        let client = Client::new();
        let response = client.get(&url).send().await?;
        response.json().await
    })
    .await
    .unwrap()
}
