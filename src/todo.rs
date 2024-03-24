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

fn write_todos(todos: Vec<TodoItem>) -> Result<usize, std::io::Error>  {
    let mut file = File::create(FILE_PATH).unwrap();
    let json = serde_json::to_string(&todos).unwrap();
    file.write(json.as_bytes())
}

// TODO - allow input with spaces surrounded by ""
pub fn add(title: &str, description: &str) {
    let mut todos = get_todos(get_file());

    if todos.iter().any(|item| item.title == title) { // If there is already todo with that name
        out::err(&format!("Item \"{}\" already exists!", title));
        return;
    }

    let new_todo = TodoItem {
        title: title.to_string(),
        description: description.to_string(),
        done: false
    };
    todos.push(new_todo);

    if let Ok(_) = write_todos(todos) { out::added(title); }
}

pub fn list() {
    let todos = get_todos(get_file());
    if todos.len() > 0 {
        for i in 0..todos.len() { out::list_item(i, &todos[i]); }
    } else {
        out::list("No todos!");
    }
    println!();
}

pub fn delete(title: &str) {
    let todos = get_todos(get_file());

    if !todos.iter().any(|item| item.title == title) { // If there are no todos with such title
        out::err(&format!("Item \"{}\" not found!", title));
        return;
    }

    let todos: Vec<TodoItem> = todos.into_iter()
        .filter(|t| t.title != title)
        .collect();

    if let Ok(_) = write_todos(todos) { out::removed(title); }
}

pub fn complete(title: &str) {
    let todos = get_todos(get_file());

    if !todos.iter().any(|t| t.title == title) { // If there are no todos with such title
        out::err(&format!("Item \"{}\" not found!", title));
        return;
    }

    let todos: Vec<TodoItem> = todos.into_iter()
        .map(|mut t| {if t.title == title {t.done = !t.done}; t})
        .collect();

    if let Ok(_) = write_todos(todos) { out::changed(title); }
}