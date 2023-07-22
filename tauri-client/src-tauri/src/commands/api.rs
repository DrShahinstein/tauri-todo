use crate::models::todo::Todo;

use reqwest::Client;
use std::env;

fn api() -> String {
    env::var("API_BASE_URL").expect("Missing env variable: API_BASE_URL")
}

pub async fn delete_todo(id: usize) -> Option<Todo> {
    let url = format!("{}/delete_todo/{}", api(), id);
    let client = Client::new();
    let response = client.delete(&url).send().await.ok()?;
    response.json().await.ok()
}

pub async fn add_todo(text: String) -> Result<String, reqwest::Error> {
    let url = format!("{}/add_todo", api());
    let client = Client::new();
    let response = client.post(&url).body(text).send().await?;
    response.text().await
}

pub async fn get_todos() -> Result<Vec<String>, reqwest::Error> {
    let url = format!("{}/get_todos", api());
    let client = Client::new();
    let response = client.get(&url).send().await?;
    response.json().await
}
