use crate::{dual::{Dual, DualComp}, error::Error, lexer::Token};

#[derive(Debug)]
pub enum Expr<T: DualComp> {
    Const(Dual<T>),
    X,

    Neg(Box<Expr<T>>),
    Add(Box<Expr<T>>, Box<Expr<T>>),
    Sub(Box<Expr<T>>, Box<Expr<T>>),
    Mul(Box<Expr<T>>, Box<Expr<T>>),
    Div(Box<Expr<T>>, Box<Expr<T>>),
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

    pub fn parse(&mut self) -> Result<Expr<T>, Error> {
        match self.parse_expr(0) {
            Some(e) => e,
            None => Err(Error::SyntaxError),
        }
    }

    fn parse_expr(&mut self, min_bp: u8) -> Option<Result<Expr<T>, Error>> {
        let current = match self.current() {
            Some(t) => *t,
            None => return None,
        };
        let mut left = match self.parse_atom(current)? {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        };

        loop {
            let op = match self.current() {
                Some(&t) => t,
                None => break,
            };
            let (left_bp, right_bp) = match binding_power(op) {
                Some(bp) => bp,
                None => break,
            };
            if left_bp < min_bp {
                break;
            }
            self.advance();

            let right = match self.parse_expr(right_bp)? {
                Ok(e) => e,
                Err(e) => return Some(Err(e)),
            };

            left = match op {
                Token::Add => Expr::Add(Box::new(left), Box::new(right)),
                Token::Sub => Expr::Sub(Box::new(left), Box::new(right)),
                Token::Mul => Expr::Mul(Box::new(left), Box::new(right)),
                Token::Div => Expr::Div(Box::new(left), Box::new(right)),
                _ => break,
            };
        }

        Some(Ok(left))
    }

    fn parse_atom(&mut self, token: Token<T>) -> Option<Result<Expr<T>, Error>> {
        match token {
            Token::Number(n) => {
                self.advance();
                Some(Ok(Expr::Const(Dual::from(n))))
            },
            Token::X => {
                self.advance();
                Some(Ok(Expr::X))
            },
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr(0)?;
                match self.current() {
                    Some(Token::RParen) => self.advance(),
                    _ => return Some(Err(Error::SyntaxError)),
                }
                Some(expr)
            },
            Token::Sub => {
                self.advance();
                let t = match self.current() {
                    Some(&t) => t,
                    None => return Some(Err(Error::SyntaxError)),
                };
                let expr = match self.parse_atom(t)? {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                Some(Ok(Expr::Neg(Box::new(expr))))
            },
            _ => Some(Err(Error::SyntaxError)),
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<&Token<T>> {
        self.input.get(self.pos)
    }
}

fn binding_power<T: DualComp>(op: Token<T>) -> Option<(u8, u8)> {
    match op {
        Token::Add | Token::Sub => Some((1, 2)),
        Token::Mul | Token::Div => Some((3, 4)),
        _ => None,
    }
}