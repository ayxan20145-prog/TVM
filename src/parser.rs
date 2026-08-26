use crate::{error::ParseError, instruction::Instruction, value::Value};

pub fn parse(source: &str) -> Result<Vec<Instruction>, ParseError> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for (line_number, line) in source.lines().enumerate() {
        let line_number = line_number + 1;

        let line = if let Some(comment_start) = line.find('#') {
            &line[..comment_start]
        } else {
            line
        };

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "push" => {
                check_args(&parts, 1, "push", line_number)?;

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
                check_args(&parts, 1, "pushstr", line_number)?;

                instructions.push(Instruction::PushStr(parts[1].to_string()));
            }
            "pushspace" => {
                check_args(&parts, 0, "pushspace", line_number)?;
                instructions.push(Instruction::PushSpace);
            }
            "pop" => {
                check_args(&parts, 0, "pop", line_number)?;

                instructions.push(Instruction::Pop);
            }
            "add" => {
                check_args(&parts, 0, "add", line_number)?;

                instructions.push(Instruction::Add);
            }
            "sub" => {
                check_args(&parts, 0, "sub", line_number)?;

                instructions.push(Instruction::Sub);
            }
            "mul" => {
                check_args(&parts, 0, "mul", line_number)?;

                instructions.push(Instruction::Mul);
            }
            "div" => {
                check_args(&parts, 0, "div", line_number)?;

                instructions.push(Instruction::Div);
            }
            "store" => {
                check_args(&parts, 1, "store", line_number)?;

                instructions.push(Instruction::Store(parts[1].to_string()));
            }
            "load" => {
                check_args(&parts, 1, "load", line_number)?;

                instructions.push(Instruction::Load(parts[1].to_string()));
            }
            "drop" => {
                check_args(&parts, 1, "drop", line_number)?;

                instructions.push(Instruction::Drop(parts[1].to_string()));
            }
            "jump" => {
                check_args(&parts, 1, "jump", line_number)?;

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
                check_args(&parts, 0, "print", line_number)?;

                instructions.push(Instruction::Print);
            }
            "println" => {
                check_args(&parts, 0, "println", line_number)?;

                instructions.push(Instruction::Println);
            }
            "read" => {
                check_args(&parts, 0, "read", line_number)?;

                instructions.push(Instruction::Read);
            }
            "jumpif" => {
                check_args(&parts, 2, "jumpif", line_number)?;

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
                        Err(_) => Value::String(String::from(parts[1])),
                    }
                };

                let address = parts[2]
                    .parse::<usize>()
                    .map_err(|_| ParseError::InvalidNumber {
                        value: String::from(parts[2]),
                        line: line_number,
                    })?;

                instructions.push(Instruction::JumpIf(value, address));
            }
            "readf" => {
                check_args(&parts, 1, "readf", line_number)?;

                let path = parts[1];

                instructions.push(Instruction::ReadF(String::from(path)));
            }
            "writef" => {
                check_args(&parts, 2, "writef", line_number)?;

                let path = parts[1];
                let content = parts[2];

                instructions.push(Instruction::WriteF(
                    String::from(path),
                    String::from(content),
                ));
            }
            "removef" => {
                check_args(&parts, 1, "removef", line_number)?;

                let path = parts[1];

                instructions.push(Instruction::RemoveF(String::from(path)));
            }
            "createdir" => {
                check_args(&parts, 1, "createdir", line_number)?;

                let path = parts[1];

                instructions.push(Instruction::CreateDir(String::from(path)));
            }
            "removedir" => {
                check_args(&parts, 1, "removedir", line_number)?;

                let path = parts[1];

                instructions.push(Instruction::RemoveDir(String::from(path)));
            }
            "stoi" => {
                check_args(&parts, 0, "stoi", line_number)?;

                instructions.push(Instruction::StoI);
            }
            "stof" => {
                check_args(&parts, 0, "stof", line_number)?;

                instructions.push(Instruction::StoF);
            }
            "itof" => {
                check_args(&parts, 0, "itof", line_number)?;

                instructions.push(Instruction::ItoF);
            }
            "itos" => {
                check_args(&parts, 0, "itos", line_number)?;

                instructions.push(Instruction::ItoS);
            }
            "ftoi" => {
                check_args(&parts, 0, "ftoi", line_number)?;

                instructions.push(Instruction::FtoI);
            }
            "ftos" => {
                check_args(&parts, 0, "ftos", line_number)?;

                instructions.push(Instruction::FtoS);
            }
            "exit" => {
                check_args(&parts, 0, "exit", line_number)?;

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
fn check_args(
    parts: &[&str],
    expected: usize,
    instruction: &str,
    line: usize,
) -> Result<(), ParseError> {
    let args = parts.len().saturating_sub(1);

    if args < expected {
        return Err(ParseError::MissingArgument {
            instruction: String::from(instruction),
            line,
        });
    }

    if args > expected {
        return Err(ParseError::UnexpectedArgument {
            instruction: String::from(instruction),
            line,
        });
    }

    Ok(())
}
