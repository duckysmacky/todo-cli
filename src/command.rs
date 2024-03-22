use std::str::SplitAsciiWhitespace;
use crate::{out, todo};

fn echo(mut args: SplitAsciiWhitespace<'_>) {
    match args.next() {
        Some(val) => out::output(val),
        None => out::err("No arguments were supplied! Usage: echo <text>"),
    }
}

fn new_todo(mut args: SplitAsciiWhitespace<'_>) {
    match args.next() {
        Some(title) => match args.next() {
            Some(desc) => todo::new_todo(title, desc),
            None => todo::new_todo(title, ""),
        },
        None => out::err("No title was supplied! Usage: new/create <title> [description]"),
    }
}

pub fn run(command: &str, args: SplitAsciiWhitespace<'_>) {
    match command {
        "echo" | "print" => echo(args),
        "new" | "create" => new_todo(args),
        _ => out::err("This command doesn't exist! Type \"help\" for full list of commands")
    }
}