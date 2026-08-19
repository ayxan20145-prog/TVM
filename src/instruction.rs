use crate::value::Value;

pub enum Instruction {
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

pub enum ReadType {
    Int,
    Float,
    String,
}
