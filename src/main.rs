mod error;
mod instruction;
mod parser;
mod value;
mod vm;

use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(name = "terb", version, about = "Terbium VM")]
struct Cli {
    input: String,
}

fn main() {
    let args = Cli::parse();

    let mut vm = vm::VM::new();

    let source = fs::read_to_string(&args.input).expect("Failed to read file");

    let instructions = match parser::parse(&source) {
        Ok(instructions) => instructions,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            return;
        }
    };

    if let Err(error) = vm.execute(&instructions) {
        eprintln!("VM error: {}", error);
    }
}
