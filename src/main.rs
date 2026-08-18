use std::{collections::HashMap, env, fs};

enum Instruction {
    Push(i32),
    Add,
    Sub,
    Mul,
    Div,
    Store(String),
    Load(String),
    Print,
    Println,
}

struct VM {
    stack: Vec<i32>,
    variables: HashMap<String, i32>,
}

impl VM {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            variables: HashMap::new(),
        }
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
                Instruction::Store(name) => {
                    let value = self.stack.pop().unwrap();

                    self.variables.insert(name.clone(), value);
                }
                Instruction::Load(name) => {
                    if let Some(&value) = self.variables.get(name) {
                        self.stack.push(value);
                    } else {
                        println!("Undefined variable: {}", name);
                    }
                }
                Instruction::Print => {
                    println!("{:?}", self.stack);
                }
                Instruction::Println => {
                    println!();
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

        if parts.is_empty() || parts[0].starts_with('#') {
            continue;
        }

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
            "store" => {
                instructions.push(Instruction::Store(parts[1].to_string()));
            }
            "load" => {
                instructions.push(Instruction::Load(parts[1].to_string()));
            }
            "print" => {
                instructions.push(Instruction::Print);
            }
            "println" => {
                instructions.push(Instruction::Println);
            }
            _ => {
                println!("Unknown instruction: {}", parts[0]);
                return;
            }
        }
    }

    vm.execute(instructions.as_slice());
}
