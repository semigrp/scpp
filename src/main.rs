use std::env;
use std::process;
use safecpp::analyzer::Analyzer;
use safecpp::parser::Parser;
use safecpp::error::Result;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    if let Err(e) = run(&args[1]) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(file_path: &str) -> Result<()> {
    let input = std::fs::read_to_string(file_path)?;
    let mut parser = Parser::new(&input);
    let ast = parser.parse()?;

    let mut analyzer = Analyzer::new();
    analyzer.analyze(&ast)?;

    println!("No memory issues detected.");
    Ok(())
}
