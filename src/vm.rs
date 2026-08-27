use crate::{error::VmError, instruction::Instruction, value::Value};
use std::{collections::HashMap, fs, io};

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
                Instruction::PushBool(value) => {
                    self.stack.push(Value::Bool(value.clone()));
                }
                Instruction::Pop => {
                    self.pop()?;
                }
                Instruction::Dup => {
                    let value = self.pop()?;

                    self.stack.push(value.clone());
                    self.stack.push(value);
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
                Instruction::Debug => {
                    print!("{:?}", self.stack);
                }
                Instruction::Read => {
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    self.stack.push(Value::String(input.trim().to_string()));
                }
                Instruction::JumpIf(value, address) => {
                    let top = self
                        .stack
                        .last()
                        .ok_or(VmError::StackUnderflow { ip: self.ip })?;

                    let equal = match (&top, value) {
                        (Value::Int(a), Value::Int(b)) => a == b,
                        (Value::Float(a), Value::Float(b)) => a == b,
                        (Value::String(a), Value::String(b)) => a == b,
                        (Value::Bool(a), Value::Bool(b)) => a == b,
                        _ => false,
                    };

                    if equal {
                        self.ip = *address;
                        continue;
                    }
                }
                Instruction::ReadF(path) => {
                    let content = fs::read_to_string(path).map_err(|_| VmError::InvalidPath {
                        path: String::from(path),
                        ip: self.ip,
                    })?;

                    self.stack.push(Value::String(content));
                }
                Instruction::WriteF(path, content) => {
                    fs::write(path, content).map_err(|_| VmError::InvalidPath {
                        path: String::from(path),
                        ip: self.ip,
                    })?;
                }
                Instruction::RemoveF(path) => {
                    fs::remove_file(path).map_err(|_| VmError::InvalidPath {
                        path: String::from(path),
                        ip: self.ip,
                    })?;
                }
                Instruction::CreateDir(path) => {
                    fs::create_dir(path).map_err(|_| VmError::InvalidPath {
                        path: String::from(path),
                        ip: self.ip,
                    })?;
                }
                Instruction::RemoveDir(path) => {
                    fs::remove_dir(path).map_err(|_| VmError::InvalidPath {
                        path: String::from(path),
                        ip: self.ip,
                    })?;
                }
                Instruction::StoI => {
                    let value = self.pop()?;

                    match value {
                        Value::String(s) => {
                            let i =
                                s.trim()
                                    .parse::<i32>()
                                    .map_err(|_| VmError::ConversionError {
                                        from: Value::String(s.clone()),
                                        to: String::from("int"),
                                        ip: self.ip,
                                    })?;

                            self.stack.push(Value::Int(i));
                        }
                        other => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("stoi"),
                                left: other,
                                right: Value::String(String::new()),
                                ip: self.ip,
                            });
                        }
                    }
                }
                Instruction::StoF => {
                    let value = self.pop()?;

                    match value {
                        Value::String(s) => {
                            let f =
                                s.trim()
                                    .parse::<f64>()
                                    .map_err(|_| VmError::ConversionError {
                                        from: Value::String(s.clone()),
                                        to: String::from("float"),
                                        ip: self.ip,
                                    })?;

                            self.stack.push(Value::Float(f));
                        }
                        other => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("stof"),
                                left: other,
                                right: Value::String(String::new()),
                                ip: self.ip,
                            });
                        }
                    }
                }
                Instruction::StoB => {
                    let value = self.pop()?;

                    match value {
                        Value::String(s) => {
                            let b =
                                s.trim()
                                    .parse::<bool>()
                                    .map_err(|_| VmError::ConversionError {
                                        from: Value::String(s.clone()),
                                        to: String::from("bool"),
                                        ip: self.ip,
                                    })?;

                            self.stack.push(Value::Bool(b));
                        }
                        other => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("stob"),
                                left: other,
                                right: Value::String(String::new()),
                                ip: self.ip,
                            });
                        }
                    }
                }
                Instruction::ItoF => {
                    let value = self.pop()?;

                    match value {
                        Value::Int(i) => {
                            self.stack.push(Value::Float(i as f64));
                        }
                        other => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("itof"),
                                left: other,
                                right: Value::String(String::new()),
                                ip: self.ip,
                            });
                        }
                    }
                }
                Instruction::ItoS => {
                    let value = self.pop()?;

                    match value {
                        Value::Int(i) => self.stack.push(Value::String(i.to_string())),
                        other => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("itos"),
                                left: other,
                                right: Value::String(String::new()),
                                ip: self.ip,
                            });
                        }
                    }
                }
                Instruction::ItoB => {
                    let value = self.pop()?;

                    match value {
                        Value::Int(i) => {
                            let b = match i {
                                0 => false,
                                _ => true,
                            };

                            self.stack.push(Value::Bool(b));
                        }
                        other => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("itob"),
                                left: other,
                                right: Value::String(String::new()),
                                ip: self.ip,
                            });
                        }
                    }
                }
                Instruction::FtoI => {
                    let value = self.pop()?;

                    match value {
                        Value::Float(f) => {
                            self.stack.push(Value::Int(f as i32));
                        }
                        other => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("ftoi"),
                                left: other,
                                right: Value::String(String::new()),
                                ip: self.ip,
                            });
                        }
                    }
                }
                Instruction::FtoS => {
                    let value = self.pop()?;

                    match value {
                        Value::Float(f) => self.stack.push(Value::String(f.to_string())),
                        other => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("ftos"),
                                left: other,
                                right: Value::String(String::new()),
                                ip: self.ip,
                            });
                        }
                    }
                }
                Instruction::FtoB => {
                    let value = self.pop()?;

                    match value {
                        Value::Float(f) => {
                            let b = match f {
                                0.0 => false,
                                _ => true,
                            };

                            self.stack.push(Value::Bool(b));
                        }
                        other => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("ftob"),
                                left: other,
                                right: Value::String(String::new()),
                                ip: self.ip,
                            });
                        }
                    }
                }
                Instruction::BtoI => {
                    let value = self.pop()?;

                    match value {
                        Value::Bool(b) => self.stack.push(Value::Int(b as i32)),
                        other => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("btoi"),
                                left: other,
                                right: Value::String(String::new()),
                                ip: self.ip,
                            });
                        }
                    }
                }
                Instruction::BtoF => {
                    let value = self.pop()?;

                    match value {
                        Value::Bool(b) => self.stack.push(Value::Float(f64::from(b))),
                        other => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("btof"),
                                left: other,
                                right: Value::String(String::new()),
                                ip: self.ip,
                            });
                        }
                    }
                }
                Instruction::BtoS => {
                    let value = self.pop()?;

                    match value {
                        Value::Bool(b) => self.stack.push(Value::String(b.to_string())),
                        other => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("btos"),
                                left: other,
                                right: Value::String(String::new()),
                                ip: self.ip,
                            });
                        }
                    }
                }
                Instruction::Eq => {
                    let b = self.pop()?;
                    let a = self.pop()?;

                    let result = match (&a, &b) {
                        (Value::Int(x), Value::Int(y)) => x == y,
                        (Value::Float(x), Value::Float(y)) => x == y,
                        (Value::Int(x), Value::Float(y)) => (*x as f64) == *y,
                        (Value::Float(x), Value::Int(y)) => *x == (*y as f64),
                        (Value::String(x), Value::String(y)) => x == y,
                        (Value::Bool(x), Value::Bool(y)) => x == y,
                        _ => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("eq"),
                                left: a,
                                right: b,
                                ip: self.ip,
                            });
                        }
                    };

                    self.stack.push(Value::Bool(result));
                }
                Instruction::Ne => {
                    let b = self.pop()?;
                    let a = self.pop()?;

                    let result = match (&a, &b) {
                        (Value::Int(x), Value::Int(y)) => x != y,
                        (Value::Float(x), Value::Float(y)) => x != y,
                        (Value::Int(x), Value::Float(y)) => (*x as f64) != *y,
                        (Value::Float(x), Value::Int(y)) => *x != (*y as f64),
                        (Value::String(x), Value::String(y)) => x != y,
                        (Value::Bool(x), Value::Bool(y)) => x != y,
                        _ => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("ne"),
                                left: a,
                                right: b,
                                ip: self.ip,
                            });
                        }
                    };

                    self.stack.push(Value::Bool(result));
                }
                Instruction::Gt => {
                    let b = self.pop()?;
                    let a = self.pop()?;

                    let result = match (&a, &b) {
                        (Value::Int(x), Value::Int(y)) => x > y,
                        (Value::Float(x), Value::Float(y)) => x > y,
                        (Value::Int(x), Value::Float(y)) => (*x as f64) > *y,
                        (Value::Float(x), Value::Int(y)) => *x > (*y as f64),
                        (Value::String(x), Value::String(y)) => x > y,
                        _ => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("gt"),
                                left: a,
                                right: b,
                                ip: self.ip,
                            });
                        }
                    };

                    self.stack.push(Value::Bool(result));
                }
                Instruction::Lt => {
                    let b = self.pop()?;
                    let a = self.pop()?;

                    let result = match (&a, &b) {
                        (Value::Int(x), Value::Int(y)) => x < y,
                        (Value::Float(x), Value::Float(y)) => x < y,
                        (Value::Int(x), Value::Float(y)) => (*x as f64) < *y,
                        (Value::Float(x), Value::Int(y)) => *x < (*y as f64),
                        (Value::String(x), Value::String(y)) => x < y,
                        _ => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("lt"),
                                left: a,
                                right: b,
                                ip: self.ip,
                            });
                        }
                    };

                    self.stack.push(Value::Bool(result));
                }
                Instruction::Ge => {
                    let b = self.pop()?;
                    let a = self.pop()?;

                    let result = match (&a, &b) {
                        (Value::Int(x), Value::Int(y)) => x >= y,
                        (Value::Float(x), Value::Float(y)) => x >= y,
                        (Value::Int(x), Value::Float(y)) => (*x as f64) >= *y,
                        (Value::Float(x), Value::Int(y)) => *x >= (*y as f64),
                        (Value::String(x), Value::String(y)) => x >= y,
                        _ => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("ge"),
                                left: a,
                                right: b,
                                ip: self.ip,
                            });
                        }
                    };

                    self.stack.push(Value::Bool(result));
                }
                Instruction::Le => {
                    let b = self.pop()?;
                    let a = self.pop()?;

                    let result = match (&a, &b) {
                        (Value::Int(x), Value::Int(y)) => x <= y,
                        (Value::Float(x), Value::Float(y)) => x <= y,
                        (Value::Int(x), Value::Float(y)) => (*x as f64) <= *y,
                        (Value::Float(x), Value::Int(y)) => *x <= (*y as f64),
                        (Value::String(x), Value::String(y)) => x <= y,
                        _ => {
                            return Err(VmError::TypeMismatch {
                                operation: String::from("le"),
                                left: a,
                                right: b,
                                ip: self.ip,
                            });
                        }
                    };

                    self.stack.push(Value::Bool(result));
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
