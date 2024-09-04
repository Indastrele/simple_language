mod logic;

use logic::ast::expression::Expression;

use crate::logic::{lexer, parser, token, token_type};

fn main() {
    // it's kinda weird, that code cannot be execute without panic if we use some same signs in a row
    let input = String::from("(500 + 700 + 3200 + 30000) * 6 + 2100");

    let tokens: Vec<token::Token> = lexer::Lexer::new(input).tokenize();

    for token in &tokens {
        println!("{}", token);
    }

    let expressions = parser::Parser::new(tokens.clone()).parse();

    for expression in &expressions {
        println!("{} = {}", expression, expression.eval());
    }
}
