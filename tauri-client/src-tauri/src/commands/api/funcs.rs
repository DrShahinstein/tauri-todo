use crate::models::todo::Todo;
use reqwest::Client;
use std::env;

fn api() -> String {
    env::var("API_BASE_URL").expect("Missing env variable: API_BASE_URL")
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
