mod logic;
mod variables;

use crate::logic::ast::{expression, statement::Statement};
use crate::logic::{lexer, parser, token, token_type};

fn main() {
    unsafe {
        match expression::VARIABLES {
            Some(_) => todo!(),
            None => expression::VARIABLES = Some(variables::variables::Variables::new()),
        }
    }
    // it's kinda weird, that code cannot be execute without panic if we use some same signs in a row
    let input = String::from("PI = 2 + 2 * PI\nword2 = 2 * - PI");

    let tokens: Vec<token::Token> = lexer::Lexer::new(input).tokenize();

    for token in &tokens {
        println!("{}", token);
    }

    let statements = parser::Parser::new(tokens.clone()).parse();

    for statement in &statements {
        statement.execute();
    }

    unsafe { println!("{:?}", expression::VARIABLES.clone().unwrap()) }
}
