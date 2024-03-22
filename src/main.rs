use std::io::stdin;

mod command;
mod out;
mod todo;

fn main() {
    out::title("--- Todo List ✅ ---");

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading input!");
        let mut input = input.trim().split_ascii_whitespace();
        
        match input.next() {
            Some(val) => match val {
                "exit" | "ext" | "close" => {
                    out::ok("Exiting todo list!");
                    break;
                },
                _ => command::run(val, input)
            }
            None => panic!("Error reading input!")
        }
    }
}
