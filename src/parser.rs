use crate::scanner::{Scanner, Token, TokenType};
use crate::value::Value;
use crate::vm::OpCode;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    /// TODO -- does this make sense semantically?
    pub fn increment(&self) -> Self {
        use Precedence::*;
        match self {
            None => Assignment,
            Assignment => Or,
            Or => And,
            And => Equality,
            Equality => Comparison,
            Comparison => Term,
            Term => Factor,
            Factor => Unary,
            Unary => Call,
            Call => Primary,
            Primary => Primary,
        }
    }
}

/// TODO -- This is an unsafe system. Is there a way to do safe constant pooling?
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct ConstantIdx(pub usize);

/// Just delegates to usize
impl Display for ConstantIdx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

pub struct Chunk {
    bytecode: Vec<OpCode>,
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

    pub fn disassemble_instruction(&self, idx: usize) {
        print!("{:04X}", idx);
        let line_number = self.lines[idx];

        // If there's multiple ops on the same line, print it with a pipe instead of line number.
        if idx > 0 && line_number == self.lines[idx - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", line_number);
        }

        let op = self.bytecode[idx];
        match op {
            OpCode::Constant(constant_idx) => self.disassemble_constant(constant_idx),
            _ => println!("{:?}", op),
        }
    }

    pub fn get_opcode(&self, idx: usize) -> OpCode {
        self.bytecode[idx]
    }

    pub fn get_constant(&self, constant_idx: ConstantIdx) -> Value {
        self.constants[constant_idx.0]
    }

    pub fn push(&mut self, op: OpCode, line: u32) {
        self.bytecode.push(op);
        self.lines.push(line);
    }

    // TODO -- Actually intern the constants
    /// Pushes a constant op with an index pointing to an element of self.constants
    pub fn push_constant(&mut self, constant: Value, line: u32) {
        self.constants.push(constant);
        let constant_idx = ConstantIdx(self.constants.len() - 1);
        let op = OpCode::Constant(constant_idx);
        self.push(op, line);
    }

    fn disassemble_constant(&self, constant_idx: ConstantIdx) {
        let value = self.constants[constant_idx.0];
        println!("{:-16} [{:4}] {}", "Constant", constant_idx, value);
    }
}

pub type ParseResult<T> = Result<T, String>;

pub struct Parser<'text> {
    scanner: Scanner<'text>,
    chunk: Chunk,

    current: Option<Token<'text>>,
    next: Token<'text>,
}

impl<'text> Parser<'text> {
    pub fn new(mut scanner: Scanner<'text>) -> Self {
        let chunk = Chunk::new();
        let next = scanner.scan(); // Start with next set to the first token.
        Self {
            scanner,
            chunk,

            current: None,
            next,
        }
    }

    pub fn compile(mut self) -> ParseResult<Chunk> {
        self.expression()?;
        self.consume(TokenType::Eof, "Expected end of expression.")?;
        self.chunk.push(OpCode::Return, 0);
        Ok(self.chunk)
    }

    // /// Prints the given error msg. Sets self.had_error true
    // fn handle_error(&mut self, msg: &str) {
    //     eprintln!("{}", msg);
    //     self.had_error = true;
    // }

    fn advance(&mut self) -> ParseResult<()> {
        self.current = self.next.into();
        let next = self.scanner.scan();
        match next.token_type {
            TokenType::Error => return Err("Error token".to_string()),
            _ => self.next = next,
        }
        Ok(())
    }

    fn consume(&mut self, token_type: TokenType, error_msg: &str) -> ParseResult<()> {
        if self.next.token_type == token_type {
            self.advance()?;
            Ok(())
        } else {
            Err(error_msg.to_string())
        }
    }

    /// '(values...)`
    fn grouping(&mut self) -> ParseResult<()> {
        // Assume '(' has already been consumed.
        self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' at end of grouping.")
    }

    fn literal(&mut self, token_type: TokenType, line: u32) -> ParseResult<()> {
        let op = match token_type {
            TokenType::False => Ok(OpCode::False),
            TokenType::Nil => Ok(OpCode::Nil),
            TokenType::True => Ok(OpCode::True),
            _ => Err(format!("Token type {:?} is not a literal.", token_type)),
        }?;
        self.chunk.push(op, line);
        Ok(())
    }

    fn number(&mut self) -> ParseResult<()> {
        let value = self
            .current
            .ok_or("number: no current")?
            .text
            .parse::<f64>()
            .or(Err("number: current can't be parsed to f64"))?;
        let value = Value::Number(value);
        self.chunk.push_constant(value, 0);
        Ok(())
    }

    fn unary(&mut self, token_type: TokenType) -> ParseResult<()> {
        self.parse_higher_precedence(Precedence::Unary)?;
        match token_type {
            TokenType::Bang => self.chunk.push(OpCode::Not, 0),
            TokenType::Minus => self.chunk.push(OpCode::Negate, 0),
            _ => panic!("Invalid unary token type: {:?}", token_type),
        }
        Ok(())
    }

    fn binary(
        &mut self,
        token_type: TokenType,
        precedence: Precedence,
        line: u32,
    ) -> ParseResult<()> {
        self.parse_higher_precedence(precedence.increment())?;

        match token_type {
            TokenType::Plus => self.chunk.push(OpCode::Add, line),
            TokenType::Minus => self.chunk.push(OpCode::Subtract, line),
            TokenType::Star => self.chunk.push(OpCode::Multiply, line),
            TokenType::Slash => self.chunk.push(OpCode::Divide, line),

            TokenType::BangEqual => {
                self.chunk.push(OpCode::Equal, line);
                self.chunk.push(OpCode::Not, line);
            }
            TokenType::EqualEqual => self.chunk.push(OpCode::Equal, line),
            TokenType::Greater => self.chunk.push(OpCode::Greater, line),
            TokenType::GreaterEqual => {
                self.chunk.push(OpCode::Less, line);
                self.chunk.push(OpCode::Not, line);
            }
            TokenType::Less => self.chunk.push(OpCode::Less, line),
            TokenType::LessEqual => {
                self.chunk.push(OpCode::Greater, line);
                self.chunk.push(OpCode::Not, line);
            }

            _ => {
                return Err(format!(
                    "Token type {:?} is not a binary operation.",
                    token_type
                ))
            }
        };
        Ok(())
    }

    fn apply_prefix_fn(&mut self, token_type: TokenType) -> ParseResult<()> {
        match token_type {
            TokenType::LeftParen => self.grouping(),
            TokenType::Minus | TokenType::Bang => self.unary(token_type),
            TokenType::Number => self.number(),
            TokenType::True | TokenType::False | TokenType::Nil => self.literal(token_type, 0),
            _ => Ok(()),
        }
    }

    fn apply_infix_fn(&mut self, token_type: TokenType) -> ParseResult<()> {
        match token_type {
            TokenType::Plus | TokenType::Minus => {
                self.binary(token_type, token_type.infix_precedence(), 0)
            }
            TokenType::Star | TokenType::Slash => {
                self.binary(token_type, token_type.infix_precedence(), 0)
            }
            TokenType::BangEqual
            | TokenType::EqualEqual
            | TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => self.binary(token_type, token_type.infix_precedence(), 0),
            _ => Ok(()),
        }
    }

    fn parse_higher_precedence(&mut self, precedence: Precedence) -> ParseResult<()> {
        self.advance()?;
        let token_type = self
            .current
            .ok_or("parse_higher_precedence: no current")?
            .token_type;
        self.apply_prefix_fn(token_type)?;

        while precedence < self.next.token_type.infix_precedence() {
            self.advance()?;
            let token_type = self
                .current
                .ok_or("parse_higher_precedence: no current")?
                .token_type;
            self.apply_infix_fn(token_type)?;
        }
        Ok(())
    }

    fn expression(&mut self) -> ParseResult<()> {
        self.parse_higher_precedence(Precedence::Assignment)
    }
}
