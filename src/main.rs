mod ast;
mod lexer;
mod parser;

use lexer::tokenize;
use parser::Parser;

fn main() {
    let input = "3 + 5 * (10 - 4)";
    let tokens = tokenize(input);
    println!("Tokens: {:?}", tokens);

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();
    println!("AST: {:?}", ast);
}
