pub trait Expression {
    fn eval(&self) -> f64;
}

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
}

impl Expression for ExpressionType {
    fn eval(&self) -> f64 {
        match self.clone() {
            ExpressionType::Number { val } => val,
            ExpressionType::Unary {
                operator,
                expression,
            } => match operator {
                '-' => -expression.eval(),
                _ => expression.eval(),
            },
            ExpressionType::Binary {
                operator,
                first_expression,
                second_expression,
            } => match operator {
                '-' => first_expression.eval() - second_expression.eval(),
                '*' => first_expression.eval() * second_expression.eval(),
                '/' => {
                    if second_expression.eval() == 0_f64 {
                        return f64::NAN;
                    }

                    first_expression.eval() / second_expression.eval()
                }
                _ => first_expression.eval() + second_expression.eval(),
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
        }
    }
}
