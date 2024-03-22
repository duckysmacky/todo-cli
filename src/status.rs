use colored::*;

pub fn ok(text: &str) {
    println!("{}", text.bright_green());
}

pub fn err(text: &str) {
    println!("{}", text.bright_red());
}