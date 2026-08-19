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
    StackUnderflow {
        ip: usize,
    },
    TypeMismatch {
        operation: String,
        left: Value,
        right: Value,
        ip: usize,
    },
    UndefinedVariable {
        name: String,
        ip: usize,
    },
    DivisionByZero {
        ip: usize,
    },
}

#[derive(Debug)]
enum ParseError {
    UnknownInstruction { instruction: String, line: usize },
    MissingArgument { instruction: String, line: usize },
    UnexpectedArgument { instruction: String, line: usize },
    InvalidReadType { value: String, line: usize },
    InvalidNumber { value: String, line: usize },
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
            VmError::StackUnderflow { ip } => {
                write!(f, "stack underflow at instruction {}", ip)
            }

            VmError::TypeMismatch {
                operation,
                left,
                right,
                ip,
            } => {
                write!(
                    f,
                    "type mismatch at instruction {}: cannot {} {} and {}",
                    ip, operation, left, right
                )
            }

            VmError::UndefinedVariable { name, ip } => {
                write!(f, "undefined variable at instruction {}: {}", ip, name)
            }

            VmError::DivisionByZero { ip } => {
                write!(f, "division by zero at instruction {}", ip)
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
        self.stack
            .pop()
            .ok_or(VmError::StackUnderflow { ip: self.ip })
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
                                ip: self.ip,
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
                                ip: self.ip,
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
                                ip: self.ip,
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
                                return Err(VmError::DivisionByZero { ip: self.ip });
                            }

                            Value::Int(a / b)
                        }

                        (Value::Float(a), Value::Float(b)) => {
                            if *b == 0.0 {
                                return Err(VmError::DivisionByZero { ip: self.ip });
                            }

                            Value::Float(a / b)
                        }

                        (Value::Int(a), Value::Float(b)) => {
                            if *b == 0.0 {
                                return Err(VmError::DivisionByZero { ip: self.ip });
                            }
                            Value::Float(*a as f64 / b)
                        }

                        (Value::Float(a), Value::Int(b)) => {
                            if *b == 0 {
                                return Err(VmError::DivisionByZero { ip: self.ip });
                            }

                            Value::Float(a / *b as f64)
                        }

                        _ => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("div"),
                                left: a,
                                right: b,
                                ip: self.ip,
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
                    let value =
                        self.variables
                            .get(name)
                            .ok_or_else(|| VmError::UndefinedVariable {
                                name: name.clone(),
                                ip: self.ip,
                            })?;

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

    let instructions = match parse(&source) {
        Ok(instructions) => instructions,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            return;
        }
    };

    if let Err(error) = vm.execute(&instructions) {
        eprintln!("VM error: {}", error);
    }
}

fn parse(source: &str) -> Result<Vec<Instruction>, ParseError> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for (line_number, line) in source.lines().enumerate() {
        let line_number = line_number + 1;

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() || parts[0].starts_with('#') {
            continue;
        }

        match parts[0] {
            "push" => {
                if parts.len() < 2 {
                    return Err(ParseError::MissingArgument {
                        instruction: String::from("push"),
                        line: line_number,
                    });
                }

                if parts.len() > 2 {
                    return Err(ParseError::UnexpectedArgument {
                        instruction: String::from("push"),
                        line: line_number,
                    });
                }

                let value = if parts[1].contains('.') {
                    match parts[1].parse::<f64>() {
                        Ok(value) => Value::Float(value),
                        Err(_) => {
                            return Err(ParseError::InvalidNumber {
                                value: String::from(parts[1]),
                                line: line_number,
                            });
                        }
                    }
                } else {
                    match parts[1].parse::<i32>() {
                        Ok(value) => Value::Int(value),
                        Err(_) => {
                            return Err(ParseError::InvalidNumber {
                                value: String::from(parts[1]),
                                line: line_number,
                            });
                        }
                    }
                };

                instructions.push(Instruction::Push(value));
            }
            "pushstr" => {
                if parts.len() < 2 {
                    return Err(ParseError::MissingArgument {
                        instruction: String::from("pushstr"),
                        line: line_number,
                    });
                }

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
                if parts.len() < 2 {
                    return Err(ParseError::MissingArgument {
                        instruction: String::from("store"),
                        line: line_number,
                    });
                }

                instructions.push(Instruction::Store(parts[1].to_string()));
            }
            "load" => {
                if parts.len() < 2 {
                    return Err(ParseError::MissingArgument {
                        instruction: String::from("load"),
                        line: line_number,
                    });
                }

                instructions.push(Instruction::Load(parts[1].to_string()));
            }
            "drop" => {
                if parts.len() < 2 {
                    return Err(ParseError::MissingArgument {
                        instruction: String::from("drop"),
                        line: line_number,
                    });
                }

                instructions.push(Instruction::Drop(parts[1].to_string()));
            }
            "jump" => {
                if parts.len() < 2 {
                    return Err(ParseError::MissingArgument {
                        instruction: String::from("jump"),
                        line: line_number,
                    });
                }

                let address = match parts[1].parse::<usize>() {
                    Ok(address) => address,
                    Err(_) => {
                        return Err(ParseError::InvalidNumber {
                            value: String::from(parts[1]),
                            line: line_number,
                        });
                    }
                };

                instructions.push(Instruction::Jump(address));
            }
            "print" => {
                instructions.push(Instruction::Print);
            }
            "println" => {
                instructions.push(Instruction::Println);
            }
            "read" => {
                if parts.len() < 2 {
                    return Err(ParseError::MissingArgument {
                        instruction: String::from("read"),
                        line: line_number,
                    });
                }

                match parts[1] {
                    "int" => instructions.push(Instruction::Read(ReadType::Int)),
                    "float" => instructions.push(Instruction::Read(ReadType::Float)),
                    "string" => instructions.push(Instruction::Read(ReadType::String)),
                    value => {
                        return Err(ParseError::InvalidReadType {
                            value: String::from(value),
                            line: line_number,
                        });
                    }
                };
            }
            "exit" => {
                instructions.push(Instruction::Exit);
            }
            instruction => {
                return Err(ParseError::UnknownInstruction {
                    instruction: String::from(instruction),
                    line: line_number,
                });
            }
        }
    }
    Ok(instructions)
}
