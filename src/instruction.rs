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
    JumpIf(Value, usize),
    ReadF(String),
    WriteF(String, String),
    RemoveF(String),
    CreateDir(String),
    RemoveDir(String),
    Exit,
}

pub enum ReadType {
    Int,
    Float,
    String,
}
