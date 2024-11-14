use crate::logic::ast::expression;

use super::{expression::Expression, statement::Statement};

pub struct AssignmentStatement {
    variable: String,
    expression: expression::ExpressionType,
}

impl AssignmentStatement {
    pub fn new(variable: String, expression: expression::ExpressionType) -> Self {
        AssignmentStatement {
            variable,
            expression,
        }
    }
}

impl Statement for AssignmentStatement {
    fn execute(&self) -> () {
        let result = match self.expression.eval() {
            Ok(res) => res,
            Err(e) => {
                println!("Programm closed with the error: {}", e);
                std::process::exit(-1);
            }
        };

        match self.variable.clone().as_str() {
            "PI" => {
                println!("Cannot change the value of constant");
                std::process::exit(-1);
            }
            "E" => {
                println!("Cannot change the value of constant");
                std::process::exit(-1);
            }
            "TAU" => {
                println!("Cannot change the value of constant");
                std::process::exit(-1);
            }
            _ => (),
        }

        unsafe {
            let mut vars = expression::VARIABLES.clone().unwrap();
            vars.set(self.variable.clone(), result);
            expression::VARIABLES = Some(vars.clone());
        };
    }
}

impl std::fmt::Display for AssignmentStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.variable, self.expression)
    }
}
