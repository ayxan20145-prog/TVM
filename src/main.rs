use std::{env, fs};

enum Instruction {
    Push(i32),
    Add,
    Sub,
    Mul,
    Div,
    Print,
}

struct VM {
    stack: Vec<i32>,
}

impl VM {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }
    fn execute(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::Push(value) => {
                    self.stack.push(*value);
                }
                Instruction::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    self.stack.push(a + b);
                }
                Instruction::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    self.stack.push(a - b);
                }
                Instruction::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    self.stack.push(a * b);
                }
                Instruction::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    self.stack.push(a / b);
                }
                Instruction::Print => {
                    println!("{:?}", self.stack);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: terb PATH");
        return;
    }

    let mut vm = VM::new();

    let source = fs::read_to_string(&args[1]).expect("Failed to read file");

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in source.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0] {
            "push" => {
                let value: i32 = parts[1].parse().unwrap();
                instructions.push(Instruction::Push(value));
            }
            "add" => {
                instructions.push(Instruction::Add);
            }
            "sub" => {
                instructions.push(Instruction::Sub);
            }
            "mul" => {
                instructions.push(Instruction::Mul);
            }
            "div" => {
                instructions.push(Instruction::Div);
            }
            "print" => {
                instructions.push(Instruction::Print);
            }
            _ => {
                println!("Unknown instruction: {}", parts[0]);
                return;
            }
        }
    }

    vm.execute(instructions.as_slice());
}
