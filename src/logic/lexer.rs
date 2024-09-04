use std::collections::HashMap;

use crate::{token::Token, token_type::TokenType};

#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
    length: i32,
    pos: i32,
    tokens: Vec<Token>,
    operators: HashMap<char, TokenType>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input.clone(),
            length: input.len() as i32,
            pos: 0_i32,
            tokens: Vec::new(),
            operators: HashMap::new(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        self.operators.insert('+', TokenType::PLUS);
        self.operators.insert('-', TokenType::MINUS);
        self.operators.insert('*', TokenType::MUL);
        self.operators.insert('/', TokenType::DIV);
        self.operators.insert('(', TokenType::LPAREN);
        self.operators.insert(')', TokenType::RPAREN);

        while self.pos < self.length {
            let current: char = self.peek(0);

            if current.is_digit(10_u32) {
                self.tokenize_number();
            } else if self.operators.contains_key(&current) {
                self.tokenize_operator();
            } else {
                self.next();
            }
        }

        return self.tokens.clone();
    }

    fn peek(&self, relative_position: i32) -> char {
        let position = self.pos + relative_position;

        if position >= self.length {
            return '\0';
        };
        self.input.clone().as_bytes()[position as usize] as char
    }

    fn next(&mut self) -> char {
        self.pos += 1;
        self.peek(0)
    }

    fn tokenize_number(&mut self) {
        let mut buffer = String::new();
        let mut current: char = self.peek(0);

        while current.is_digit(10) {
            buffer += current.to_string().as_str();
            current = self.next();
        }

        self.add_token(&TokenType::NUMBER, Some(buffer));
    }

    fn tokenize_operator(&mut self) {
        self.add_token(
            self.operators.clone().get(&self.peek(0)).unwrap(),
            Some(self.peek(0).to_string()),
        );
        self.next();
    }

    fn add_token(&mut self, token_type: &TokenType, text: Option<String>) {
        self.tokens
            .push(Token::new(*token_type, text.clone().unwrap()));
    }
}
