use std::io::stdin;

use crate::{dual::Dual, lexer::Lexer};

mod dual;
mod lexer;
mod error;
mod parser;

fn main() {
    let input = get_input();

    let mut lexer = Lexer::new(input.chars().collect());
    let tokens = match lexer.tokenize::<f64>() {
        Ok(t) => t,
        Err(e) => {
            println!("{e}");
            return;
        },
    };
    println!("{:?}", tokens)
}

fn get_input() -> String {
    let mut s = String::new();
    if matches!(stdin().read_line(&mut s), Err(_)) {
        s = get_input();
    }
    s.trim().to_string()
}
