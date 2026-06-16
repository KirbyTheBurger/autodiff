use crate::{dual::DualComp, error::Error};

#[derive(Debug)]
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
            if c.is_whitespace() {
                continue;
            }

            tokens.push(self.read_token(*c)?);
        }

        Ok(tokens)
    }

    fn read_token<T: DualComp>(&mut self, c: char) -> Result<Token<T>, Error> {
        match c {
            c if c.is_numeric() => {
                let mut s = String::new();
                s.push(c);
                self.advance();

                while let Some(&c) = self.current() {
                    self.advance();

                    if c.is_numeric() || c == '.' {
                        s.push(c);
                    } else {
                        break;
                    }
                }

                match s.parse::<T>() {
                    Ok(f) => Ok(Token::Number(f)),
                    Err(_) => Err(Error::SyntaxError),
                }
            }
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