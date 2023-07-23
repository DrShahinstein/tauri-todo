import React, { useState, useEffect } from "react";
import { tauri } from "@tauri-apps/api";
import "./App.css";

export default function App(): React.ReactElement {
  const [todos, setTodos] = useState<string[]>([]);
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
    setTodos([...todos, inputValue]);
    tauri
      .invoke("command_add_todo", { text: inputValue })
      .then((res) => console.log(res))
      .catch((err) => console.error(err));
    setInputValue("");
  };

  const deleteTodo = (index: number) => {
    const updatedTodos = todos.filter((_, i) => i !== index);
    setTodos(updatedTodos);
    tauri
      .invoke("command_delete_todo", { id: index })
      .then((res) => console.log(res))
      .catch((err) => console.error(err));
  };

  const fetchTodos = () => {
    tauri
      .invoke<string[]>("command_get_todos")
      .then((res) => setTodos(res))
      .catch((err) => console.error("Error fetching todos: ", err));
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
        {todos.map((todo, index) => (
          <li key={index} className="todo-element">
            <span className="todo-element--text">{todo}</span>
            <button
              onClick={() => deleteTodo(index)}
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
