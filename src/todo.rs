use std::fs::File;
use std::io::{BufReader, Write};
use serde::{Deserialize, Serialize};

use crate::out;

static FILE_PATH: &str = "./todos.json";

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    title: String,
    description: String,
    done: bool
}

fn get_file() -> File {
    let mut file = File::open(FILE_PATH);
    if let Err(_) = file {
        let _ = File::create(FILE_PATH);
        file = File::open(FILE_PATH);
    }
    file.unwrap()
}

fn get_todos(file: File) -> Vec<TodoItem> {
    let reader = BufReader::new(file);
    let mut todos: Vec<TodoItem> = Vec::new();
    if let Ok(val) = serde_json::from_reader(reader) {
        todos = val;
    }
    todos
}

fn write_todos(mut file: File, todos: Vec<TodoItem>) {
    let json = serde_json::to_string(&todos).unwrap();
    match file.write(json.as_bytes()) {
        Ok(_) => out::added(&format!("{:?}", todos.last().unwrap().title)),
        Err(_) => out::err("Error writing to todos.json!")
    }
}

pub fn new_todo(title: &str, description: &str) {
    let new_todo = TodoItem {
        title: title.to_string(),
        description: description.to_string(),
        done: false
    };

    let file = get_file();

    let mut todos = get_todos(file);
    todos.push(new_todo.clone());

    let file = File::create(FILE_PATH).unwrap();
    write_todos(file, todos);
}

pub fn list_todos() {
    let file = get_file();
    let todos = get_todos(file);
    for i in 0..todos.len() {
        out::list(i, &todos[i].title, &todos[i].description, todos[i].done);
    }
}