# Todo App Using Rocket and Tauri

A simple Todo App built using Rocket framework for the backend and Tauri for the frontend. The app allows users to add, delete, and view their todos with a Rocket based server 🚀

## Features

- Add a new todo by entering the text in the input field and clicking "Add" button.
- Delete a todo by clicking the "Delete" button next to the todo.
- View all todos in a list.
- Keep track of what are added and deleted using a Rocket server.

## Prerequisites

Before running the app, make sure you have the following dependencies installed:

- [rust & cargo](https://rustup.rs/)
- [yarn](https://classic.yarnpkg.com/lang/en/docs/install/#debian-stable)
- [tauri-cli](https://tauri.app/v1/api/cli/)

You can use tauri-cli within yarn as it already involves it as a dependency. But, you can also install it on your system independently.

## Installation

1. Clone the repository: `git clone https://www.github.com/DrShahinstein/rocket-tauri-todo.git`
2. Change your dir to the project dir: `cd rocket-tauri-todo/`
3. Setup `API_BASE_URL` env variable: `echo "API_BASE_URL='http://127.0.0.1:8000/api'" > .env`
4. Setup and run rocket-server: `cargo r --manifest-path rocket-server/Cargo.toml`
5. Setup frontend: `yarn --cwd tauri-client/`
6. Setup and run tauri using yarn: `yarn --cwd tauri-client/ tauri dev`
   - Using an independent build of tauri: `cd tauri-client/ && cargo tauri dev`

## Build

1. Change your dir to `tauri-client/`
2. Build tauri app using yarn: `yarn tauri build`
   - `cargo tauri build`
3. You can specialize `API_BASE_URL` for bundles if you'd like (same goes for releases)

Check out [here](https://tauri.app/v1/guides/building/) for more details.

## API Endpoints

- `GET /get_todos`: Fetch all todos.
- `POST /add_todo`: Add a new todo. Payload: `{ "text": "Your todo text" }`.
- `DELETE /delete_todo/<id>`: Delete a todo with the given ID.

## Contributing

Contributions are warmly welcome as this project was built for learning and exploring Rust purposes. If you find a bug or have an idea for an improvement, feel free to open an issue or submit a pull request.

## License

This project is licensed under the [Do What the Fuck You Want to Public License](https://github.com/DrShahinstein/rocket-tauri-todo/blob/main/LICENSE)
