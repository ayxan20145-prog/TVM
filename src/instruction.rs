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
    Jump(JumpTarget),
    Print,
    Println,
    Debug,
    Read,
    JumpIf(Value, JumpTarget),
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
    Label(String),
    Call(String),
    Ret,
    DropLabel(String),
    Sh,
    Exit,
}

pub enum JumpTarget {
    Address(usize),
    Label(String),
}
