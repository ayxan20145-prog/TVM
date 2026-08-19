use std::{collections::HashMap, env, fmt, fs, io};

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
    Read(ReadType),
    Exit,
}

#[derive(Clone, Debug)]
enum Value {
    Int(i32),
    Float(f64),
    String(String),
}

enum ReadType {
    Int,
    Float,
    String,
}

enum VmError {
    StackUnderflow,
    TypeMismatch {
        operation: String,
        left: Value,
        right: Value,
    },
    UndefinedVariable(String),
    DivisionByZero,
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

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::StackUnderflow => {
                write!(f, "stack underflow")
            }

            VmError::TypeMismatch {
                operation,
                left,
                right,
            } => {
                write!(f, "cannot {} {} and {}", operation, left, right)
            }

            VmError::UndefinedVariable(name) => {
                write!(f, "undefined variable: {}", name)
            }

            VmError::DivisionByZero => {
                write!(f, "division by zero")
            }
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
    fn pop(&mut self) -> Result<Value, VmError> {
        self.stack.pop().ok_or(VmError::StackUnderflow)
    }
    fn execute(&mut self, instructions: &[Instruction]) -> Result<(), VmError> {
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
                    self.pop()?;
                }
                Instruction::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;

                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a + b),

                        (Value::Float(a), Value::Float(b)) => Value::Float(a + b),

                        (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 + b),

                        (Value::Float(a), Value::Int(b)) => Value::Float(a + *b as f64),

                        (Value::String(a), Value::String(b)) => {
                            Value::String(format!("{}{}", a, b))
                        }

                        _ => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("add"),
                                left: a,
                                right: b,
                            });
                        }
                    };

                    self.stack.push(result);
                }
                Instruction::Sub => {
                    let b = self.pop()?;
                    let a = self.pop()?;

                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a - b),

                        (Value::Float(a), Value::Float(b)) => Value::Float(a - b),

                        (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 - b),

                        (Value::Float(a), Value::Int(b)) => Value::Float(a - *b as f64),

                        (Value::String(a), Value::String(b)) => Value::String(a.replace(b, "")),

                        _ => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("sub"),
                                left: a,
                                right: b,
                            });
                        }
                    };

                    self.stack.push(result);
                }
                Instruction::Mul => {
                    let b = self.pop()?;
                    let a = self.pop()?;

                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a * b),

                        (Value::Float(a), Value::Float(b)) => Value::Float(a * b),

                        (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 * b),

                        (Value::Float(a), Value::Int(b)) => Value::Float(a * *b as f64),

                        _ => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("mul"),
                                left: a,
                                right: b,
                            });
                        }
                    };

                    self.stack.push(result);
                }
                Instruction::Div => {
                    let b = self.pop()?;
                    let a = self.pop()?;

                    let result = match (&a, &b) {
                        (Value::Int(a), Value::Int(b)) => {
                            if *b == 0 {
                                return Err(VmError::DivisionByZero);
                            }

                            Value::Int(a / b)
                        }

                        (Value::Float(a), Value::Float(b)) => {
                            if *b == 0.0 {
                                return Err(VmError::DivisionByZero);
                            }

                            Value::Float(a / b)
                        }

                        (Value::Int(a), Value::Float(b)) => {
                            if *b == 0.0 {
                                return Err(VmError::DivisionByZero);
                            }
                            Value::Float(*a as f64 / b)
                        }

                        (Value::Float(a), Value::Int(b)) => {
                            if *b == 0 {
                                return Err(VmError::DivisionByZero);
                            }

                            Value::Float(a / *b as f64)
                        }

                        _ => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("div"),
                                left: a,
                                right: b,
                            });
                        }
                    };

                    self.stack.push(result);
                }
                Instruction::Store(name) => {
                    let value = self.pop()?;

                    self.variables.insert(name.clone(), value);
                }
                Instruction::Load(name) => {
                    let value = self
                        .variables
                        .get(name)
                        .ok_or_else(|| VmError::UndefinedVariable(name.clone()))?;

                    self.stack.push(value.clone());
                }
                Instruction::Drop(name) => {
                    self.variables.remove(name);
                }
                Instruction::Jump(address) => {
                    self.ip = *address;
                    continue;
                }
                Instruction::Print => {
                    print!("{}", self.pop()?);
                }
                Instruction::Println => {
                    println!();
                }
                Instruction::Read(value) => match value {
                    ReadType::Int => {
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");

                        let input: i32 = match input.trim().parse() {
                            Ok(e) => e,
                            Err(e) => {
                                println!("Error: {}", e);
                                break;
                            }
                        };
                        self.stack.push(Value::Int(input));
                    }
                    ReadType::Float => {
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");

                        let input: f64 = match input.trim().parse() {
                            Ok(e) => e,
                            Err(e) => {
                                println!("Error: {}", e);
                                break;
                            }
                        };
                        self.stack.push(Value::Float(input));
                    }
                    ReadType::String => {
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");

                        self.stack.push(Value::String(input.trim().to_string()));
                    }
                },
                Instruction::Exit => {
                    break;
                }
            }
            self.ip += 1;
        }
        Ok(())
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
            "read" => {
                match parts[1] {
                    "int" => instructions.push(Instruction::Read(ReadType::Int)),
                    "float" => instructions.push(Instruction::Read(ReadType::Float)),
                    "string" => instructions.push(Instruction::Read(ReadType::String)),
                    _ => {
                        println!("Unknown type");
                        break;
                    }
                };
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

    if let Err(error) = vm.execute(&instructions) {
        eprintln!("VM error: {}", error);
    }
}
