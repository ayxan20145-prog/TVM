enum Instruction {
    Add,
    Print,
}

struct VM {
    stack: Vec<i32>,
}

impl VM {
    fn new() {
        let mut vm = VM { stack: Vec::new() };
    }
}

fn main() {
    let mut vm = VM::new();
}
