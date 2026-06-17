use crate::{dual::DualComp, error::Error};

#[derive(Debug)]
pub enum Token<T: DualComp> {
    Number(T),

    Add,
    Sub,
    Mul,
    Div,

    LBracket,
    RBracket,

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
                self.advance();
                continue;
            }

            tokens.push(self.read_token(*c)?);
            self.advance();
        }

        Ok(tokens)
    }

    fn read_token<T: DualComp>(&mut self, c: char) -> Result<Token<T>, Error> {
        match c {
            '+' => Ok(Token::Add),
            '-' => Ok(Token::Sub),
            '*' => Ok(Token::Mul),
            '/' => Ok(Token::Div),
            '(' => Ok(Token::LBracket),
            ')' => Ok(Token::RBracket),
            'x' => Ok(Token::X),

            c if c.is_numeric() => {
                let mut s = String::new();
                s.push(c);

                while let Some(&c) = self.peek() {
                    if c.is_numeric() || c == '.' {
                        self.advance();
                        s.push(c);
                    } else {
                        break;
                    }
                }

                match s.parse::<T>() {
                    Ok(f) => Ok(Token::Number(f)),
                    Err(_) => Err(Error::SyntaxError),
                }
            },

            _ => Err(Error::UnknownChar),
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<&char> {
        self.input.get(self.pos)
    }

    fn peek(&self) -> Option<&char> {
        self.input.get(self.pos + 1)
    }
}