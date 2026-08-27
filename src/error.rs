use crate::value::Value;
use std::fmt;

pub enum VmError {
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
    InvalidPath {
        path: String,
        ip: usize,
    },
    ConversionError {
        from: Value,
        to: String,
        ip: usize,
    },
}

pub enum ParseError {
    UnknownInstruction { instruction: String, line: usize },
    MissingArgument { instruction: String, line: usize },
    UnexpectedArgument { instruction: String, line: usize },
    InvalidNumber { value: String, line: usize },
    InvalidBool { value: String, line: usize },
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

            VmError::InvalidPath { path, ip } => {
                write!(f, "invalid path at instruction {}: {}", ip, path)
            }
            VmError::ConversionError { from, to, ip } => {
                write!(
                    f,
                    "conversion error at instruction {}: cannot convert {} to {}",
                    ip, from, to
                )
            }
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnknownInstruction { instruction, line } => {
                write!(f, "unknown instruction at line {}: {}", line, instruction)
            }

            ParseError::MissingArgument { instruction, line } => {
                write!(f, "missing argument at line {}: {}", line, instruction)
            }

            ParseError::UnexpectedArgument { instruction, line } => {
                write!(f, "unexpected argument at line {}: {}", line, instruction)
            }

            ParseError::InvalidNumber { value, line } => {
                write!(f, "invalid number at line {}: {}", line, value)
            }

            ParseError::InvalidBool { value, line } => {
                write!(f, "invalid boolean at line {}: {}", line, value)
            }
        }
    }
}
