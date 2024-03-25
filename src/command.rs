use std::fs;
use std::slice::Iter;

use crate::out;
use crate::todo;

fn help() {
    println!("{}", "\nList of all commands");
    out::list("help - shows the list of all commands");
    out::list("echo | print <input> - returns input (used for debug)");
    out::list("new | create | add <title> [description] - add new todo item");
    out::list("delete | del | remove <title> - delete a todo");
    out::list("check | done | mark | complete <title> - mark a todo as completed/not completed");
    out::list("edit <item title> <title/description> <value> - edit a todo item's description or title");
    out::list("list | todos - list all todo items");
    out::list("clear - clears all todos (deletes todos.json)");
    out::list("exit | ext | close - exits the program\n");
}

fn clear_all() {
    match fs::remove_file("./todos.json") {
        Err(e) => out::err(&format!("Error removing todos.json: {e}")),
        Ok(_) => out::removed("All todos")
    }
}

fn echo(mut args: Iter<String>) {
    match args.next() {
        Some(val) => out::output(val),
        None => out::err("No text was supplied! Usage: echo <text>"),
    }
}

fn add_todo(mut args: Iter<String>) {
    match args.next() {
        None => out::err("No item title was entered! Usage: new | create | add <title> [description]"),
        Some(title) => match args.next() {
            None => todo::add(title, ""),
            Some(desc) => todo::add(title, desc)
        }
    }
}

fn delete_todo(mut args: Iter<String>) {
    match args.next() {
        None => out::err("No item title was entered! Usage: delete | del | remove <title>"),
        Some(title) => todo::delete(title)
    }
}

fn complete_todo(mut args: Iter<String>) {
    match args.next() {
        None => out::err("No item title was entered! Usage: check | done | mark | complete <title>"),
        Some(title) => todo::complete(title)
    }
}

fn edit_todo(mut args: Iter<String>) {
    match args.next() {
        None => out::err("No item title was entered! Usage: edit <title> <title/description> <value>"),
        Some(title) => match args.next() {
            None => out::err("No valid element was entered! Usage: edit <title> <title/description> <value>"),
            Some(element) => match args.next() {
                None => out::err("No value was entered! Usage: edit <title> <title/description> <value>"),
                Some(value) => todo::edit(title, element, value)
            }
        }
    }
}

pub fn run(command: &str, args: Iter<String>) {
    match command {
        "help" => help(),
        "echo" | "print" => echo(args),
        "new" | "create" | "add" => add_todo(args),
        "delete" | "del" | "remove" => delete_todo(args),
        "check" | "done" | "mark" | "complete" => complete_todo(args),
        "edit" => edit_todo(args),
        "list" | "todos" => todo::list(),
        "clear" => clear_all(),
        _ => out::err("This command doesn't exist! Type \"help\" for full list of commands")
    }
}