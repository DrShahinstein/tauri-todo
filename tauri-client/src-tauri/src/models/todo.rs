use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Todo {
    pub text: String,
    pub id: usize,
}
