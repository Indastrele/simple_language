use crate::{token::Token, token_type::TokenType};

use super::ast::expression::ExpressionType;

pub struct Parser {
    tokens: Option<Vec<Token>>,
    size: i32,
    pos: i32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: Some(tokens.clone()),
            size: tokens.len() as i32,
            pos: 0_i32,
        }
    }

    pub fn parse(&mut self) -> Vec<ExpressionType> {
        let mut result = Vec::new();

        while !self.compare(TokenType::EOF) {
            result.push(self.expression().ok().unwrap())
        }

        result
    }

    fn expression(&mut self) -> Result<ExpressionType, &'static str> {
        self.additive()
    }

    fn additive(&mut self) -> Result<ExpressionType, &'static str> {
        let mut result = self.multiplicative();

        loop {
            if self.compare(TokenType::PLUS) {
                result = Ok(ExpressionType::Binary {
                    operator: '+',
                    first_expression: Box::new(result.ok().unwrap()),
                    second_expression: Box::new(self.multiplicative().ok().unwrap()),
                });

                continue;
            }

            if self.compare(TokenType::MINUS) {
                result = Ok(ExpressionType::Binary {
                    operator: '-',
                    first_expression: Box::new(result.ok().unwrap()),
                    second_expression: Box::new(self.multiplicative().ok().unwrap()),
                });

                continue;
            }

            break;
        }

        result
    }

    fn multiplicative(&mut self) -> Result<ExpressionType, &'static str> {
        let mut result = self.unary();

        loop {
            if self.compare(TokenType::MUL) {
                result = Ok(ExpressionType::Binary {
                    operator: '*',
                    first_expression: Box::new(result.ok().unwrap()),
                    second_expression: Box::new(self.unary().ok().unwrap()),
                });

                continue;
            }

            if self.compare(TokenType::DIV) {
                result = Ok(ExpressionType::Binary {
                    operator: '/',
                    first_expression: Box::new(result.ok().unwrap()),
                    second_expression: Box::new(self.unary().ok().unwrap()),
                });

                continue;
            }

            break;
        }

        result
    }

    // there is a bug, that makes program panic when we try to calculate some multiple expressions with the same sign
    fn unary(&mut self) -> Result<ExpressionType, &'static str> {
        if self.compare(TokenType::MINUS) {
            return Ok(ExpressionType::Unary {
                operator: '-',
                expression: Box::new(self.primary().ok().unwrap()),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<ExpressionType, &'static str> {
        let current = self.get(0);

        if self.compare(TokenType::NUMBER) {
            return Ok(ExpressionType::Number {
                val: current.get_text().parse().unwrap(),
            });
        }

        if self.compare(TokenType::LPAREN) {
            let result = self.expression();
            self.compare(TokenType::RPAREN);
            return result;
        }

        Err("")
    }

    fn compare(&mut self, token_type: TokenType) -> bool {
        let token = self.get(0);

        if token_type != token.clone().get_token_type() {
            return false;
        }

        self.pos += 1;
        true
    }

    fn get(&self, relative_position: i32) -> Token {
        let position = self.pos + relative_position;

        if position >= self.size {
            return Token::new(TokenType::EOF, "".to_string());
        }

        return match self.tokens.clone() {
            Some(tokens) => tokens.get(position as usize).unwrap().clone(),
            None => Token::new(TokenType::EOF, "".to_string()),
        };
    }
}
