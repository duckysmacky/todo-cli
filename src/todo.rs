use std::{fs::File, io::Write};
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use crate::out;

const FILE_PATH: &str = "./todos.json";

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub title: String,
    pub description: String,
    pub done: bool
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

// TODO - allow input with spaces surrounded by ""
pub fn add(title: &str, description: &str) {
    let new_todo = TodoItem {
        title: title.to_string(),
        description: description.to_string(),
        done: false
    };

    let file = get_file();

    let mut todos = get_todos(file);
    todos.push(new_todo);

    let mut file = File::create(FILE_PATH).unwrap();
    let json = serde_json::to_string(&todos).unwrap();
    match file.write(json.as_bytes()) {
        Ok(_) => out::added(&format!("{:?}", todos.last().unwrap().title)),
        Err(_) => out::err("Error writing to todos.json!")
    }
}

pub fn list() {
    println!();
    let file = get_file();
    let todos = get_todos(file);
    for i in 0..todos.len() {
        out::list_item(i, &todos[i]);
    }
    println!();
}

pub fn complete(title: &str) {
    let file = get_file();
    let todos = get_todos(file);

    let mut new_todos = Vec::new();
    for mut todo_item in todos {
        if todo_item.title == title.to_string() {
            todo_item.done = !todo_item.done;
        }
        new_todos.push(todo_item);
    }

    let mut file = File::create(FILE_PATH).unwrap();
    let json = serde_json::to_string(&new_todos).unwrap();
    match file.write(json.as_bytes()) {
        Ok(_) => out::changed(&format!("{:?}", new_todos.last().unwrap().title)),
        Err(_) => out::err("Error writing to todos.json!")
    }
}