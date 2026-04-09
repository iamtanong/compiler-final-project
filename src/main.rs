mod codegen;
mod lexer;
mod parser;

use lexer::tokenize;
use parser::Parser;
use std::env;
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 && args.len() != 3 {
        eprintln!(
            "Usage: {} <input_file> or \n\t{} <input_file> <output_file>",
            args[0], args[0]
        );
        std::process::exit(1);
    }

    let input_file = &args[1];

    let output_file = if args.len() == 3 {
        Some(&args[2])
    } else {
        None
    };

    // Read input file
    let source = fs::read_to_string(input_file)
        .map_err(|e| format!("Failed to read input file '{}': {}", input_file, e))?;

    // Tokenize
    let tokens = tokenize(&source)?;

    // Parse and generate B-code
    let mut parser = Parser::new(tokens);
    let b_code = parser.parse()?;

    // Write output file or print to stdout
    if let Some(output_file) = output_file {
        let mut output = fs::File::create(output_file)
            .map_err(|e| format!("Failed to create output file '{}': {}", output_file, e))?;

        output
            .write_all(b_code.as_bytes())
            .map_err(|e| format!("Failed to write to output file: {}", e))?;
    } else {
        println!("{}", b_code);
    }

    Ok(())
}
