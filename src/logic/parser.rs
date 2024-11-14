use crate::{token::Token, token_type::TokenType};

use super::ast::{assignment_statement::AssignmentStatement, expression::ExpressionType};

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

    pub fn parse(&mut self) -> Vec<AssignmentStatement> {
        let mut result = Vec::new();

        while !self.compare(TokenType::EOF) {
            result.push(self.statement().ok().unwrap())
        }

        result
    }

    fn statement(&mut self) -> Result<AssignmentStatement, &'static str> {
        self.assignment_statement()
    }

    fn assignment_statement(&mut self) -> Result<AssignmentStatement, &'static str> {
        let curr = self.get(0);
        if self.compare(TokenType::WORD) && self.get(0).get_token_type() == TokenType::EQ {
            let var = curr.clone().get_text();
            let _ = self.consume(TokenType::EQ);
            let statement = match self.expression() {
                Ok(data) => data,
                Err(e) => {
                    println!("{e}");
                    std::process::exit(-1);
                }
            };
            return Ok(AssignmentStatement::new(var, statement));
        }

        Err("No such statement")
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

        if self.compare(TokenType::WORD) {
            return Ok(ExpressionType::Variable {
                name: current.get_text(),
            });
        }

        if self.compare(TokenType::LPAREN) {
            let result = self.expression();
            self.compare(TokenType::RPAREN);
            return result;
        }

        println!("Primary statement error: {}", current.clone());
        Err("")
    }

    fn consume(&mut self, token_type: TokenType) -> Result<Token, &'static str> {
        let curr = self.get(0);
        if token_type == curr.get_token_type() {
            self.pos += 1;
            return Ok(curr);
        }

        Err("No such token")
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
