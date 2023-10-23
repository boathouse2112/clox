use crate::scanner::Scanner;
use crate::value::Value;

/// This is an unsafe system. Is there a way to do safe constant pooling?
pub type ConstantIdx = usize;

pub struct Chunk {
    bytecode: Vec<u8>,
    lines: Vec<u32>,
    constants: Vec<Value>,
}

impl Chunk {
    fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    fn push(&mut self, byte: u8, line: u32) {
        self.bytecode.push(byte);
        self.lines.push(line);
    }
}

// pub struct Parser {
//
// }
//
// impl Parser {
//     pub fn compile() -> Chunk {
//         let scanner = Scanner::new();
//         let chunk = Chunk::new();
//
//     }
// }
