use crate::{
    error::VmError,
    instruction::{Instruction, ReadType},
    value::Value,
};
use std::{collections::HashMap, io};

pub struct VM {
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
    ip: usize,
}

impl VM {
    pub fn new() -> Self {
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
    pub fn execute(&mut self, instructions: &[Instruction]) -> Result<(), VmError> {
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
                                return Err(VmError::InvalidInput { input, ip: self.ip });
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
                Instruction::JumpIf(value, address) => {
                    let top = self.pop()?;

                    let equal = match (&top, value) {
                        (Value::Int(a), Value::Int(b)) => a == b,
                        (Value::Float(a), Value::Float(b)) => a == b,
                        (Value::String(a), Value::String(b)) => a == b,
                        _ => false,
                    };

                    if equal {
                        self.ip = *address;
                        continue;
                    }
                }
                Instruction::Exit => {
                    break;
                }
            }
            self.ip += 1;
        }
        Ok(())
    }
}
