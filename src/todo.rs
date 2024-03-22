use std::fs::File;
use std::io::{BufReader, Write};
use serde::{Deserialize, Serialize};

use crate::out;

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    title: String,
    description: String,
    done: bool
}

pub fn new_todo(title: &str, description: &str) {
    let filename = "./todos.json";
    let new_todo = TodoItem {
        title: title.to_string(),
        description: description.to_string(),
        done: false
    };

    let mut file = File::open(filename);
    if let Err(_) = file {
        let _ = File::create(filename);
        file = File::open(filename);
    }

    let reader = BufReader::new(file.unwrap());
    let mut todos: Vec<TodoItem> = Vec::new();
    if let Ok(val) = serde_json::from_reader(reader) {
        todos = val;
    }
    todos.push(new_todo.clone());
    println!("{:?}", todos);

    let mut file = File::create(filename).unwrap();
    let json = serde_json::to_string(&todos).unwrap();
    match file.write(json.as_bytes()) {
        Ok(_) => out::added(&format!("{:?}", new_todo)),
        Err(_) => out::err("Error writing to todos.json!")
    }
}