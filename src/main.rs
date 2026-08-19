mod error;
mod instruction;
mod parser;
mod value;
mod vm;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: terb PATH");
        return;
    }

    let mut vm = vm::VM::new();

    let source = fs::read_to_string(&args[1]).expect("Failed to read file");

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
