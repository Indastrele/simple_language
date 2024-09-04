use std::fmt;
use std::fmt::{Display, Formatter};

use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    text: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, text: String) -> Token {
        Token {
            token_type,
            text: Some(text.clone()),
        }
    }

    pub fn get_token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn set_token_type(&mut self, token_type: TokenType) {
        self.token_type = token_type;
    }

    pub fn get_text(&self) -> String {
        match self.text.clone() {
            Some(string) => string.clone(),
            None => "".to_string(),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = Some(text.clone());
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.text.clone() {
            Some(text) => write!(f, "{} {}", self.token_type, text),
            None => write!(f, "{}", self.token_type),
        }
    }
}
