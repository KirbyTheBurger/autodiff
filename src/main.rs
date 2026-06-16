use crate::{dual::Dual, lexer::Lexer};

mod dual;
mod lexer;
mod error;

fn main() {
    let mut lexer = Lexer::new("5.24".chars().collect());
    let tokens = match lexer.tokenize::<f64>() {
        Ok(t) => t,
        Err(e) => {
            println!("{e}");
            return;
        },
    };
    println!("{:?}", tokens)
}
