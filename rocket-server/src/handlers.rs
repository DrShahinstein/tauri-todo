use crate::todo::{Todo, TodosState};
use rocket::serde::json::Json;
use rocket::State;

#[delete("/delete_todo/<id>")]
pub fn delete_todo(id: usize, todos: &State<TodosState>) -> Option<Json<Todo>> {
    let mut todos = todos.lock().expect("Failed to lock todos");
    let index = todos.iter().position(|todo| todo.id == id);
    if let Some(index) = index {
        let deleted_todo = todos.remove(index);
        Some(Json(deleted_todo))
    } else {
        None
    }
}

#[post("/add_todo", data = "<text>")]
pub fn add_todo(text: String, todos: &State<TodosState>) -> &'static str {
    let mut todos = todos.lock().expect("Failed to lock todos");
    let next_id = todos.len() + 1;
    todos.push(Todo {
        id: next_id,
        text: text.clone(),
    });
    "Todo added successfully!"
}

#[get("/get_todos")]
pub fn get_todos(todos: &State<TodosState>) -> Json<Vec<String>> {
    let todos = todos.lock().expect("Failed to lock todos");
    let todos_json: Vec<String> = todos.iter().map(|todo| todo.text.clone()).collect();
    Json(todos_json)
}
