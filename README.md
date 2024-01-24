# Todo App Using Tauri and Rocket

A todo app working on desktop using Tauri. This is been a project of mine that I've slightly discovered Tauri. 

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

If your build somehow fails, try to re-build again using the `--verbose` flag like this: `yarn tauri build --verbose`
This way, you can keep track of what's going wrong. In my case, where I use an Arch machine, I didn't have the packages of `wget` and `libicui18n.so.72` so my build was not successful. When I had them, everything was okay.

Check out [the original guide](https://tauri.app/v1/guides/building/) of tauri for more details regarding build.

## API Endpoints

- `GET /get_todos`: Fetch all todos.
- `POST /add_todo`: Add a new todo. Payload: `{ "text": "Your todo text" }`.
- `DELETE /delete_todo/<id>`: Delete a todo with the given ID.
