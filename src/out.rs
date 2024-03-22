#![allow(unused)]

use colored::*;

pub fn ok(text: &str) {
    println!("{}", text.green());
}

pub fn output(text: &str) {
    println!("{}", text.bright_cyan());
}

pub fn info(text: &str) {
    println!("{}", text.blue());
}

pub fn err(text: &str) {
    println!("{}", text.red());
}

pub fn title(text: &str) {
    println!("{}", text.blue());
}

pub fn added(element: &str) {
    println!("{} {}", "(+)".bright_green(), element.bright_green());
}

pub fn removed(element: &str) {
    println!("{} {}", "(-)".bright_red(), element.bright_red());
}

pub fn list(index: usize, title: &str, desc: &str, done: bool) {
    let content = &format!(
        "[{}] {} - {}",
        if done { "X" } else { "O" }, title, desc
    );
    println!("{}", content.bright_black());
}