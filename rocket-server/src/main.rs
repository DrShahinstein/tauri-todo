#[macro_use]
extern crate rocket;

mod handlers;
mod todo;

use handlers::{add_todo, delete_todo, get_todos};
use std::sync::{Arc, Mutex};
use todo::Todo;

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Todo App! Use /api to manage todos."
}

#[launch]
fn rocket() -> _ {
    let todos = Arc::new(Mutex::new(Vec::<Todo>::new()));
    rocket::build()
        .manage(todos)
        .mount("/", routes![index])
        .mount("/api", routes![get_todos, add_todo, delete_todo])
}
