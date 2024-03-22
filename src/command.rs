use std::str::SplitAsciiWhitespace;
use crate::out;
use crate::todo;

fn help() {
    println!("{}", "\nList of all commands");
    out::list("help - shows the list of all commands");
    out::list("echo | print <input> - returns input (used for debug)");
    out::list("new | create <title> [description] - add new todo item");
    out::list("list | todos - list all todo items");
    out::list("exit | ext | close - exits the program\n");
}

fn echo(mut args: SplitAsciiWhitespace<'_>) {
    match args.next() {
        Some(val) => out::output(val),
        None => out::err("No arguments were supplied! Usage: echo <text>"),
    }
}

fn add_todo(mut args: SplitAsciiWhitespace<'_>) {
    match args.next() {
        Some(title) => match args.next() {
            Some(desc) => todo::new_todo(title, desc),
            None => todo::new_todo(title, ""),
        },
        None => out::err("No title was entered! Usage: new/create <title> [description]"),
    }
}

pub fn run(command: &str, args: SplitAsciiWhitespace<'_>) {
    match command {
        "help" => help(),
        "echo" | "print" => echo(args),
        "new" | "create" | "add" => add_todo(args),
        "list" | "todos" => todo::list_todos(),
        _ => out::err("This command doesn't exist! Type \"help\" for full list of commands")
    }
}