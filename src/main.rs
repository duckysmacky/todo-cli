use std::io::stdin;
use colored::*;

mod command;
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
        
        match input.next() {
            Some(val) => match val {
                "exit" | "ext" | "close" => {
                    status::ok("Exiting todo list!");
                    break;
                },
                _ => command::run(val, input)
            }
            None => panic!("Error reading input!")
        }
    }
}
