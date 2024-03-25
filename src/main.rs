use std::io::stdin;

mod command;
mod out;
mod todo;

fn main() {
    out::title("--- Todo List ✅ ---");

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading input!");
        let input = input.trim();

        let mut args: Vec<String> = Vec::new();
        let mut arg = String::new();
        let mut q = false;
        for c in input.chars() {
            match c {
                ' ' => { // Space separator
                    if q {
                        arg.push(c);
                    } else {
                        args.push(arg.clone());
                        arg.clear();
                    }
                },
                '\"' => q = !q, // Toggle read between quotations
                _ => arg.push(c)
            }
        }
        args.push(arg.clone());

        match args[0].as_str() {
            "exit" | "ext" | "close" => {
                out::ok("Exiting todo list!");
                break;
            },
            _ => command::run(args[0].as_str(), args[1..].iter())
        }
    }
}
