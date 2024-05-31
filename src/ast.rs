use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Variable(String),
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Assign {
        name: String,
        expr: Box<Expr>,
    },
}
