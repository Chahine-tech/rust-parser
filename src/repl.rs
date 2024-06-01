use crate::evaluator::evaluate;
use crate::lexer::tokenize;
use crate::parser::Parser;
use std::collections::HashMap;
use std::io::{self, Write};

pub fn repl() {
    let mut context = HashMap::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            break;
        }

        let tokens = tokenize(input);
        let mut parser = Parser::new(&tokens);
        let ast = parser.parse();

        match evaluate(&ast, &mut context) {
            value => println!("Evaluated result: {}", value),
        }
    }
}
