use std::{collections::HashMap, env, fmt, fs};

enum Instruction {
    Push(Value),
    PushStr(String),
    PushSpace,
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Store(String),
    Load(String),
    Drop(String),
    Jump(usize),
    Print,
    Println,
    Exit,
}

#[derive(Clone)]
enum Value {
    Int(i32),
    Float(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
        }
    }
}

struct VM {
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
    ip: usize,
}

impl VM {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            variables: HashMap::new(),
            ip: 0,
        }
    }
    fn execute(&mut self, instructions: &[Instruction]) {
        while self.ip < instructions.len() {
            match &instructions[self.ip] {
                Instruction::Push(value) => {
                    self.stack.push(value.clone());
                }
                Instruction::PushStr(value) => {
                    self.stack.push(Value::String(value.clone()));
                }
                Instruction::PushSpace => {
                    self.stack.push(Value::String(String::from(" ")));
                }
                Instruction::Pop => {
                    self.stack.pop().unwrap();
                }
                Instruction::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a + b),

                        (Value::Float(a), Value::Float(b)) => Value::Float(a + b),

                        (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 + b),

                        (Value::Float(a), Value::Int(b)) => Value::Float(a + *b as f64),

                        (Value::String(a), Value::String(b)) => {
                            Value::String(format!("{}{}", a, b))
                        }

                        _ => panic!("Cant add: {} and {}", a, b),
                    };

                    self.stack.push(result);
                }
                Instruction::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a - b),

                        (Value::Float(a), Value::Float(b)) => Value::Float(a - b),

                        (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 - b),

                        (Value::Float(a), Value::Int(b)) => Value::Float(a - *b as f64),

                        (Value::String(a), Value::String(b)) => Value::String(a.replace(b, "")),

                        _ => panic!("Cant sub: {} and {}", a, b),
                    };

                    self.stack.push(result);
                }
                Instruction::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a * b),

                        (Value::Float(a), Value::Float(b)) => Value::Float(a * b),

                        (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 * b),

                        (Value::Float(a), Value::Int(b)) => Value::Float(a * *b as f64),

                        _ => panic!("Cant mul: {} and {}", a, b),
                    };

                    self.stack.push(result);
                }
                Instruction::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a / b),

                        (Value::Float(a), Value::Float(b)) => Value::Float(a / b),

                        (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 / b),

                        (Value::Float(a), Value::Int(b)) => Value::Float(a / *b as f64),

                        _ => panic!("Cant div: {} and {}", a, b),
                    };

                    self.stack.push(result);
                }
                Instruction::Store(name) => {
                    let value = self.stack.pop().unwrap();

                    self.variables.insert(name.clone(), value);
                }
                Instruction::Load(name) => {
                    if let Some(value) = self.variables.get(name) {
                        self.stack.push(value.clone());
                    } else {
                        panic!("Undefined variable: {}", name);
                    }
                }
                Instruction::Drop(name) => {
                    self.variables.remove(name);
                }
                Instruction::Jump(address) => {
                    self.ip = *address;
                    continue;
                }
                Instruction::Print => {
                    print!("{}", self.stack.pop().unwrap());
                }
                Instruction::Println => {
                    println!();
                }
                Instruction::Exit => {
                    break;
                }
            }
            self.ip += 1;
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
                let value = if parts[1].contains('.') {
                    Value::Float(parts[1].parse().unwrap())
                } else {
                    Value::Int(parts[1].parse().unwrap())
                };

                instructions.push(Instruction::Push(value));
            }
            "pushstr" => {
                instructions.push(Instruction::PushStr(parts[1].to_string()));
            }
            "pushspace" => {
                instructions.push(Instruction::PushSpace);
            }
            "pop" => {
                instructions.push(Instruction::Pop);
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
            "drop" => {
                instructions.push(Instruction::Drop(parts[1].to_string()));
            }
            "jump" => {
                let address: usize = parts[1].parse().unwrap();
                instructions.push(Instruction::Jump(address));
            }
            "print" => {
                instructions.push(Instruction::Print);
            }
            "println" => {
                instructions.push(Instruction::Println);
            }
            "exit" => {
                instructions.push(Instruction::Exit);
            }
            _ => {
                println!("Unknown instruction: {}", parts[0]);
                return;
            }
        }
    }

    vm.execute(instructions.as_slice());
}
