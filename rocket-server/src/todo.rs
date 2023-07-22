use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
pub struct Todo {
    pub text: String,
    pub id: usize,
}

pub type TodosState = Arc<Mutex<Vec<Todo>>>;
