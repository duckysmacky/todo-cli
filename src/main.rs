use std::{io::stdin, str::SplitAsciiWhitespace};
use colored::*;

mod status;

#[allow(unused)]
#[derive(Debug)]
struct TodoItem {
    title: String,
    description: String,
    done: bool
}

fn main() {
    println!("{}", "Todo list".bright_green());

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading input!");
        let mut input = input.trim().split_ascii_whitespace();
        
        match input.next().unwrap() {
            "exit" => {
                status::ok("Exiting todo list!");
                break;
            },
            "print" => println!("Input: {}", input.next().unwrap()),
            _ => status::err("This command doesn't exist!")
        }
    }
}
