use crate::ast::Expr;

pub fn compile(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => format!("PUSH {}", n),
        Expr::Variable(name) => format!("LOAD {}", name),
        Expr::BinaryOp { left, op, right } => {
            let left_code = compile(left);
            let right_code = compile(right);
            let op_code = match op {
                crate::lexer::Token::Plus => "ADD",
                crate::lexer::Token::Minus => "SUB",
                crate::lexer::Token::Multiply => "MUL",
                crate::lexer::Token::Divide => "DIV",
                _ => panic!("Unsupported operator"),
            };
            format!("{}\n{}\n{}", left_code, right_code, op_code)
        }
        Expr::Assign { name, expr } => {
            let expr_code = compile(expr);
            format!("{}\nSTORE {}", expr_code, name)
        }
    }
}
