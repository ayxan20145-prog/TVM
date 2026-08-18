enum Instruction {
    Push(i32),
    Add,
    Print,
}

struct VM {
    stack: Vec<i32>,
}

impl VM {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }
    fn execute(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::Push(value) => {
                    self.stack.push(*value);
                }
                Instruction::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    self.stack.push(a + b);
                }
                Instruction::Print => {
                    println!("{:?}", self.stack);
                }
            }
        }
    }
}

fn main() {
    let mut vm = VM::new();

    let instructions = vec![
        Instruction::Push(5),
        Instruction::Push(3),
        Instruction::Add,
        Instruction::Print,
    ];

    vm.execute(instructions.as_slice());
}
