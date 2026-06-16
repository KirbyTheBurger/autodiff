use crate::{dual::DualComp, error::Error};

pub enum Token<T: DualComp> {
    Number(T),

    Add,
    Sub,
    Mul,
    Div,

    LBrace,
    RBrace,

    X,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer {
        Lexer {
            input,
            pos: 0,
        }
    }

    pub fn tokenize<T: DualComp>(&mut self) -> Result<Vec<Token<T>>, Error> {
        let mut tokens = vec![];

        while let Some(c) = self.current() {
            if let Some(t) = self.read_token(*c)? {
                tokens.push(t);
            }
        }

        Ok(tokens)
    }

    fn read_token<T: DualComp>(&mut self, c: char) -> Result<Option<Token<T>>, Error> {
        match c {
            c if c.is_whitespace() => Ok(None),
            _ => todo!()
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<&char> {
        self.input.get(self.pos)
    }
}