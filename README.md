# Rust Todo List CLI

A simple command-line todo list application written in Rust. This application allows you to manage your tasks with basic CRUD operations and persistent storage.

## Features

- Add new tasks
- List all tasks with completion status
- Mark tasks as completed
- Remove tasks
- Persistent storage using JSON
- Simple command-line interface

## Requirements

- Rust (edition 2021)
- Cargo

## Dependencies

- serde (1.0) - For serialization/deserialization
- serde_json (1.0) - For JSON handling

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd todo_list
```

2. Build the project:

```bash
cargo build --release
```

3. Run the application:

```bash
cargo run
```

## Usage

The application presents a menu with the following options:

1. Add a task - Create a new task
2. List all tasks - Display all tasks with their status
3. Mark a task as done - Complete a task
4. Remove a task - Delete a task

- Type 'q' to exit

### Data Storage

Tasks are automatically saved to `tasks.json` in the application directory. The file is created automatically if it doesn't exist.

## License

MIT
