use crate::value::Value;

pub enum Instruction {
    Push(Value),
    PushStr(String),
    PushBool(bool),
    PushSpace,
    Pop,
    Dup,
    Add,
    Sub,
    Mul,
    Div,
    Inc,
    Dec,
    Store(String),
    Load(String),
    Drop(String),
    Jump(usize),
    Print,
    Println,
    Debug,
    Read,
    JumpIf(Value, usize),
    ReadF,
    WriteF,
    RemoveF,
    CreateDir,
    RemoveDir,
    StoI,
    StoF,
    StoB,
    ItoF,
    ItoS,
    ItoB,
    FtoI,
    FtoS,
    FtoB,
    BtoI,
    BtoF,
    BtoS,
    Eq, // ==
    Ne, // !=
    Gt, // >
    Lt, // <
    Ge, // >=
    Le, // <=
    And,
    Or,
    Not,
    Exit,
}
