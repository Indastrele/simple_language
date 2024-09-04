use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,

    NUMBER,
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            TokenType::PLUS => write!(f, "PLUS"),
            TokenType::MINUS => write!(f, "MINUS"),
            TokenType::MUL => write!(f, "MUL"),
            TokenType::DIV => write!(f, "DIV"),
            TokenType::LPAREN => write!(f, "LPAREN"),
            TokenType::RPAREN => write!(f, "RPAREN"),
            TokenType::NUMBER => write!(f, "NUMBER"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}
