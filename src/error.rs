use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    SyntaxError,
    MathError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MathError => write!(f, "Math Error"),
            Error::SyntaxError => write!(f, "Syntax Error"),
        }
    }
}

impl std::error::Error for Error {}