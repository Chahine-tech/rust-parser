use rust_parser::repl::repl;
use rust_parser::run_example;

fn main() {
    // Example input
    let input = "x = 3 + 5 * (10 - 4)";
    run_example(input);

    // Start REPL
    println!("\nEntering REPL mode. Type your expressions:");
    repl();
}
