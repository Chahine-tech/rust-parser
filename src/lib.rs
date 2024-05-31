pub mod ast;
pub mod compiler;
pub mod evaluator;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod stdlib;

use compiler::compile;
use evaluator::evaluate;
use interpreter::interpret;
use lexer::tokenize;
use parser::Parser;
use std::collections::HashMap;
use stdlib::stdlib;

pub fn run_example(input: &str) {
    // Tokenize the input
    let tokens = tokenize(input);
    println!("Tokens: {:?}", tokens);

    // Parse the tokens into an AST
    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();
    println!("AST: {:?}", ast);

    // Evaluate the AST
    let mut eval_context = HashMap::new();
    let eval_result = evaluate(&ast, &mut eval_context);
    println!("Evaluated result: {}", eval_result);

    // Interpret the AST
    let mut interp_context = HashMap::new();
    let interp_result = interpret(&ast, &mut interp_context);
    println!("Interpreted result: {}", interp_result);

    // Compile the AST
    let compiled_code = compile(&ast);
    println!("Compiled code:\n{}", compiled_code);

    // Example usage of the standard library
    let lib = stdlib();
    if let Some(abs) = lib.get("abs") {
        println!("abs(-10) = {}", abs(-10));
    }
}
