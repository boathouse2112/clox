mod stack;

use crate::parser::{Chunk, ConstantIdx};
use crate::value::Value;
use crate::vm::InterpretError::RuntimeError;
use std::ops::{Add, Div, Mul, Sub};

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

pub type InterpretResult = Result<(), InterpretError>;
#[derive(Debug)]
pub enum InterpretError {
    RuntimeError(String),
}

trait Peekable<T> {
    fn peek(&self) -> Option<&T> {
        self.peek_distance(0)
    }

    fn peek_distance(&self, distance: usize) -> Option<&T>;
}

impl<T> Peekable<T> for Vec<T> {
    fn peek_distance(&self, distance: usize) -> Option<&T> {
        self.get(self.len() - 1 - distance)
    }
}

/// VM that runs a chunk
pub struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Vm {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    /// Interpret the chunk
    pub fn interpret(mut self) -> InterpretResult {
        println!("==== execution ====");

        loop {
            print!("         ");
            for value in self.stack.iter() {
                print!("[{}]", value);
            }
            println!();
            self.chunk.disassemble_instruction(self.ip);

            let op = self.read_byte();
            match op {
                OpCode::Nop => {}
                OpCode::Constant(constant_idx) => {
                    let constant = self.chunk.get_constant(constant_idx);
                    self.stack.push(constant);
                }
                OpCode::Nil => self.stack.push(Value::Nil),
                OpCode::True => self.stack.push(Value::Bool(true)),
                OpCode::False => self.stack.push(Value::Bool(false)),
                OpCode::Negate => self.negate()?,
                OpCode::Add => self.binary(f64::add)?,
                OpCode::Subtract => self.binary(f64::sub)?,
                OpCode::Multiply => self.binary(f64::mul)?,
                OpCode::Divide => self.binary(f64::div)?,
                OpCode::Return => return self.ret(),
            }
        }
    }

    fn runtime_error(msg: String) -> InterpretResult {
        Err(RuntimeError(msg))
    }

    fn read_byte(&mut self) -> OpCode {
        let op = self.chunk.get_opcode(self.ip);
        self.ip += 1;
        op
    }

    fn negate(&mut self) -> InterpretResult {
        let negated = self
            .stack
            .pop()
            .expect("Negate called without value on stack");
        match negated {
            Value::Number(n) => self.stack.push(Value::Number(-n)),
            _ => {
                return Self::runtime_error(format!(
                    "Value {} to be negated is not a number",
                    negated
                ));
            }
        }
        Ok(())
    }

    fn binary(&mut self, op: fn(f64, f64) -> f64) -> InterpretResult {
        let second = self
            .stack
            .pop()
            .expect("Binary operation called without two operands");
        let first = self
            .stack
            .pop()
            .expect("Binary operation called without two operands");
        match (first, second) {
            (Value::Number(first), Value::Number(second)) => {
                let result = Value::Number(op(first, second));
                self.stack.push(result);
            }
            _ => {
                return Self::runtime_error(
                    "Values to be binary operated are not both numbers".to_string(),
                );
            }
        }
        Ok(())
    }

    fn ret(&mut self) -> InterpretResult {
        let return_value = self
            .stack
            .pop()
            .expect("Returned with no value on the stack");
        println!("{}", return_value);
        Ok(())
    }
}
