mod stack;

use crate::parser::{Chunk, ConstantIdx};
use crate::vm::stack::Stack;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum OpCode {
    Nop, // Do nothing

    Constant(ConstantIdx),
    // LongConstant(LongConstantIdx),
    Nil,
    True,
    False,

    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
}

// impl From<OpCode> for u8 {
//     fn from(op: OpCode) -> Self {
//         op as u8
//     }
// }

/// VM that runs a chunk
pub struct Vm {
    chunk: Chunk,
    ip: u8,
    stack: Stack,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Vm {
            chunk,
            ip: 0,
            stack: Stack::new(),
        }
    }

    /// Interpret the chunk
    pub fn interpret(self) {
        println!("==== execution ====");

        loop {
            print!("    ");
            for value in self.stack.iter() {
                println!("{:?}", value);
            }
        }
    }
}
