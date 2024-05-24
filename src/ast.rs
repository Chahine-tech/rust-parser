use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
}
