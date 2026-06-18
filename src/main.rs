use std::io::stdin;

use crate::{lexer::Lexer, parser::Parser};

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
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let expr = match parser.parse() {
        Ok(e) => e,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };
    println!("{:?}", expr);
}

fn get_input() -> String {
    let mut s = String::new();
    if matches!(stdin().read_line(&mut s), Err(_)) {
        s = get_input();
    }
    s.trim().to_string()
}
