#![allow(unused)]

use colored::*;

pub fn ok(text: &str) {
    println!("{}", text.green());
}

pub fn out(text: &str) {
    println!("{}", text.bright_cyan());
}

pub fn info(text: &str) {
    println!("{}", text.blue());
}

pub fn err(text: &str) {
    println!("{}", text.red());
}

pub fn added(element: &str) {
    println!("[+] {}", element.bright_green());
}

pub fn removed(element: &str) {
    println!("[-] {}", element.bright_red());
}