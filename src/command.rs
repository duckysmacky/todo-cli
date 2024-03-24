use std::str::SplitAsciiWhitespace;
use crate::out;
use crate::todo;

fn help() {
    println!("{}", "\nList of all commands");
    out::list("help - shows the list of all commands");
    out::list("echo | print <input> - returns input (used for debug)");
    out::list("new | create | add <title> [description] - add new todo item");
    out::list("check | done | mark | complete <title> - mark a todo as completed/not completed");
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
        None => out::err("No title was entered! Usage: new | create | add <title> [description]"),
        Some(title) => match args.next() {
            None => todo::add(title, ""),
            Some(desc) => todo::add(title, desc)
        }
    }
}

fn delete_todo(mut args: SplitAsciiWhitespace<'_>) {
    match args.next() {
        None => out::err("No item was entered! Usage: delete | del | remove <title>"),
        Some(title) => todo::delete(title)
    }
}

fn complete_todo(mut args: SplitAsciiWhitespace<'_>) {
    match args.next() {
        None => out::err("No item was entered! Usage: check | done | mark | complete <title>"),
        Some(title) => todo::complete(title)
    }
}

pub fn run(command: &str, args: SplitAsciiWhitespace<'_>) {
    match command {
        "help" => help(),
        "echo" | "print" => echo(args),
        "new" | "create" | "add" => add_todo(args),
        "delete" | "del" | "remove" => delete_todo(args),
        "check" | "done" | "mark" | "complete" => complete_todo(args),
        "list" | "todos" => todo::list(),
        _ => out::err("This command doesn't exist! Type \"help\" for full list of commands")
    }
}