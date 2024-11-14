use crate::variables::variables::Variables;

pub trait Expression {
    fn eval(&self) -> Result<f64, &'static str>;
}

pub static mut VARIABLES: Option<Variables> = None;

#[derive(Debug, Clone)]
pub enum ExpressionType {
    Number {
        val: f64,
    },
    Unary {
        operator: char,
        expression: Box<ExpressionType>,
    },
    Binary {
        operator: char,
        first_expression: Box<ExpressionType>,
        second_expression: Box<ExpressionType>,
    },
    Variable {
        name: String,
    },
}

impl Expression for ExpressionType {
    fn eval(&self) -> Result<f64, &'static str> {
        match self.clone() {
            ExpressionType::Number { val } => Ok(val),
            ExpressionType::Unary {
                operator,
                expression,
            } => match operator {
                '-' => Ok(-expression.eval().ok().unwrap()),
                '+' => expression.eval(),
                _ => Err("Unknwown symbol"),
            },
            ExpressionType::Binary {
                operator,
                first_expression,
                second_expression,
            } => {
                let f_expr = match first_expression.eval() {
                    Ok(val) => val,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(-1);
                    }
                };

                let s_expr = match second_expression.eval() {
                    Ok(val) => val,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(-1);
                    }
                };
                match operator {
                    '-' => Ok(f_expr - s_expr),
                    '*' => Ok(f_expr * s_expr),
                    '/' => {
                        if s_expr == 0_f64 {
                            return Ok(f64::NAN);
                        }

                        Ok(f_expr / s_expr)
                    }
                    '+' => Ok(f_expr + s_expr),
                    _ => Err("Unknown symbol"),
                }
            }
            ExpressionType::Variable { name } => unsafe {
                Ok(VARIABLES.clone().unwrap().get(name))
            },
        }
    }
}

impl std::fmt::Display for ExpressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.clone() {
            ExpressionType::Number { val } => write!(f, "{}", val),
            ExpressionType::Unary {
                operator,
                expression,
            } => match operator {
                '-' => write!(f, "({}{})", operator, expression),
                _ => write!(f, "{}", expression),
            },
            ExpressionType::Binary {
                operator,
                first_expression,
                second_expression,
            } => write!(f, "{} {} {}", first_expression, operator, second_expression),
            ExpressionType::Variable { name } => unsafe {
                write!(f, "{}", VARIABLES.clone().unwrap().get(name))
            },
        }
    }
}
