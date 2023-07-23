use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub text: String,
    pub id: usize,
}

pub type TodosState = Arc<Mutex<Vec<Todo>>>;
