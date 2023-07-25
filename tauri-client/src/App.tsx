import React, { useState, useEffect } from "react";
import { tauri } from "@tauri-apps/api";
import "./App.css";

type TodoObj = { id: number; text: string }[];

export default function App(): React.ReactElement {
  const [todos, setTodos] = useState<TodoObj>([]);
  const [inputValue, setInputValue] = useState<string>("");

  useEffect(() => {
    fetchTodos();
  }, []);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setInputValue(e.target.value);
  };

  const addTodo = (e: React.FormEvent) => {
    e.preventDefault();
    if (inputValue.trim() === "") return;

    const newTodo: TodoObj = [{ text: inputValue, id: todos.length + 1 }];
    setTodos(todos.concat(newTodo));

    tauri
      .invoke("command_add_todo", { text: inputValue })
      .then((res) => console.log(res))
      .catch((err) => console.error(err));
    setInputValue("");
  };

  const deleteTodo = (id: number) => {
    console.log("id: %d", id);
    const updatedTodos = todos.filter((todo) => todo.id !== id);
    setTodos(updatedTodos);

    tauri
      .invoke("command_delete_todo", { id })
      .then((res) => console.log(res))
      .catch((err) => console.error(err));
  };

  const fetchTodos = () => {
    tauri
      .invoke<TodoObj>("command_get_todos")
      .then((res) => setTodos(res))
      .catch((err) => console.error(err));
  };

  return (
    <div className="todo-app">
      <h1 className="todo-title">Todo App</h1>
      <form className="todo-form" onSubmit={addTodo}>
        <input
          type="text"
          value={inputValue}
          onChange={handleInputChange}
          placeholder="Add a todo..."
          className="todo-form--input"
        />
        <button type="submit" className="todo-form--btn">
          Add
        </button>
      </form>
      <ul className="todo-container">
        {todos.map((todo) => (
          <li key={todo.id} className="todo-element">
            <span className="todo-element--text">{todo.text}</span>
            <button
              onClick={() => deleteTodo(todo.id)}
              className="todo-element--delete-btn"
            >
              Delete
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}
