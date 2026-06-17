use crate::{dual::{Dual, DualComp}, lexer::Token};

pub enum Expr<T: DualComp> {
    Number(Dual<T>)
}

pub struct Parser<T: DualComp> {
    input: Vec<Token<T>>,
    pos: usize,
}

impl<T: DualComp> Parser<T> {
    pub fn new(input: Vec<Token<T>>) -> Parser<T> {
        Parser {
            input,
            pos: 0,
        }
    }

    fn parse(&mut self) -> Expr<T> {
        todo!()
    }

    fn parse_atom(&mut self) -> Expr<T> {
        todo!()
    }

    fn binding_power(op: &Token<T>) -> (u8, u8) {
        match op {
            Token::Add | Token::Sub => (1, 2),
            Token::Mul | Token::Div => (3, 4),
            _ => (0, 0),
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<&Token<T>> {
        self.input.get(self.pos)
    }
}