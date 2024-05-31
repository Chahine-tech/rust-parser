use crate::ast::Expr;
use crate::lexer::Token;

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Expr {
        let expr = self.parse_expression();
        if let Some(Token::Assign) = self.current_token() {
            self.pos += 1;
            if let Expr::Variable(name) = expr {
                let value = self.parse_expression();
                return Expr::Assign {
                    name,
                    expr: Box::new(value),
                };
            } else {
                panic!("Expected variable name before '='");
            }
        }
        expr
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        while let Some(op) = self.current_token().cloned() {
            match op {
                Token::Plus | Token::Minus => {
                    self.pos += 1;
                    let right = self.parse_factor();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        while let Some(op) = self.current_token().cloned() {
            match op {
                Token::Multiply | Token::Divide => {
                    self.pos += 1;
                    let right = self.parse_primary();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current_token().cloned() {
            Some(Token::Number(n)) => {
                self.pos += 1;
                Expr::Number(n)
            }
            Some(Token::Identifier(ref name)) => {
                self.pos += 1;
                Expr::Variable(name.clone())
            }
            Some(Token::LParen) => {
                self.pos += 1;
                let expr = self.parse_expression();
                if let Some(Token::RParen) = self.current_token() {
                    self.pos += 1;
                    expr
                } else {
                    panic!("Expected closing parenthesis");
                }
            }
            _ => panic!("Unexpected token"),
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
}
