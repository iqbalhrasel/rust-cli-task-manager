# Rust CLI project for Daily Task Manager
A CLI based daily task manager

## Features
- Add task
  ```sh
  add "this is a new task"
  ```
- Delete task
  ```sh
  del 1
  ```
- Done task
  ```sh
  done --id 1 --done y
  ```
- Update task description
  ```sh
  upd --id 1 --desc "update task with this"
  ```
- Get all tasks
  ```sh
  list
  ```

## Tech stack
- Rust 1.96
- Clap
- Sqlite3 db
- Rusqlite for db operation
