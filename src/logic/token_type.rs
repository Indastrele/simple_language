#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    EQ,

    NUMBER,
    WORD,
    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TokenType::PLUS => write!(f, "PLUS"),
            TokenType::MINUS => write!(f, "MINUS"),
            TokenType::MUL => write!(f, "MUL"),
            TokenType::DIV => write!(f, "DIV"),
            TokenType::LPAREN => write!(f, "LPAREN"),
            TokenType::RPAREN => write!(f, "RPAREN"),
            TokenType::EQ => write!(f, "EQ"),
            TokenType::NUMBER => write!(f, "NUMBER"),
            TokenType::WORD => write!(f, "WORD"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}
