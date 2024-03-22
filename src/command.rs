use std::str::SplitAsciiWhitespace;
use crate::status;

fn echo(mut args: SplitAsciiWhitespace<'_>) {
    match args.next() {
        Some(val) => status::out(val),
        None => status::err("No arguments were supplied! Usage: echo <text>"),
    }
}

pub fn run(command: &str, args: SplitAsciiWhitespace<'_>) {
    match command {
        "echo" => echo(args),
        _ => status::err("This command doesn't exist! Type \"help\" for full list of commands")
    }
}