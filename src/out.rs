#![allow(unused)]

use colored::*;

use crate::todo::TodoItem;

pub fn ok(text: &str) { println!("\n{}\n", text.green()); }

pub fn err(text: &str) { println!("\n{}\n", text.red()); }

pub fn title(text: &str) { println!("{}", text.blue()); }

pub fn output(text: &str) { println!("\n{}\n", text.bright_cyan()); }

pub fn added(element: &str) { println!("\n{}\n", format!("(+) {} added", element).bright_green()); }

// TODO - change the icon for "changed"
pub fn changed(element: &str) { println!("\n{}\n", format!("(*) {}", element).bright_yellow()); }

pub fn removed(element: &str) { println!("\n{}\n", format!("(-) {} removed", element).bright_red()); }

pub fn info(text: &str) { println!("\n{}", format!("(?) {}", text).bright_blue()); }

pub fn list(text: &str) { println!("• {}", text.bright_black()); }

pub fn list_item(index: usize, item: &TodoItem) {
    println!("{}", 
        if item.description.is_empty() {
            format!(
                "[{}] {}",
                if item.done {"X"} else {" "},
                item.title
            )
        } else {
            format!(
                "[{}] {} - {}",
                if item.done {"X"} else {" "},
                item.title,
                item.description
            )
        }.bright_black()
    );
}