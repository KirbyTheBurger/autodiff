use std::io::stdin;

use clap::Parser as CliParser;

use crate::{evaluator::Evaluator, lexer::Lexer, parser::Parser};

mod dual;
mod lexer;
mod error;
mod parser;
mod evaluator;

#[derive(CliParser)]
#[command(name = "autodiff", about = "Differentiates a function using dual numbers")]
struct Args {
    // Print debug info
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    println!("Enter your function:");
    let input = get_input();

    let mut lexer = Lexer::new(input.chars().collect());
    let tokens = match lexer.tokenize::<f64>() {
        Ok(t) => t,
        Err(e) => {
            println!("{e}");
            return;
        },
    };

    if args.debug {
        println!("Tokens: {:?}", tokens);
    }

    let mut parser = Parser::new(tokens);
    let expr = match parser.parse() {
        Ok(e) => e,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };
    
    if args.debug {
        println!("Expression: {:?}", expr);
    }

    println!("Enter the x position of the derivative you want:");
    let pos;
    loop {
        match get_input().parse::<f64>() {
            Ok(n) => {
                pos = n;
                break;
            },
            Err(e) => println!("Failed to parse number: {e}. Please enter the position again"),
        }
    }

    let eval = Evaluator::new(pos);
    let result = eval.evaluate(expr);
    println!("Derivative at x = {}, y = {} is {}", pos, result.real, result.dual);
}

fn get_input() -> String {
    let mut s = String::new();
    if matches!(stdin().read_line(&mut s), Err(_)) {
        s = get_input();
    }
    s.trim().to_string()
}
