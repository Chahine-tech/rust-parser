use crate::ast::Expr;
use std::collections::HashMap;

pub fn interpret(expr: &Expr, context: &mut HashMap<String, i64>) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Variable(name) => *context
            .get(name)
            .expect(&format!("Variable {} not defined", name)),
        Expr::BinaryOp { left, op, right } => {
            let left_val = interpret(left, context);
            let right_val = interpret(right, context);
            match op {
                crate::lexer::Token::Plus => left_val + right_val,
                crate::lexer::Token::Minus => left_val - right_val,
                crate::lexer::Token::Multiply => left_val * right_val,
                crate::lexer::Token::Divide => left_val / right_val,
                _ => panic!("Unsupported operator"),
            }
        }
        Expr::Assign { name, expr } => {
            let value = interpret(expr, context);
            context.insert(name.clone(), value);
            value
        }
    }
}
