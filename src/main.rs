mod parser;
mod scanner;
mod value;
mod vm;

use crate::parser::{ParseResult, Parser};
use crate::scanner::{Scanner, TokenType};
use crate::vm::{InterpretError, InterpretResult, Vm};
use std::{env, io};

fn repl() {
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        line.clear();
        _ = stdin.read_line(&mut line).expect("Invalid input");

        let scanner = Scanner::new(&line);
        let parser = Parser::new(scanner);
        let chunk = match parser.compile() {
            Ok(chunk) => chunk,
            Err(msg) => {
                println!("{}", msg);
                break;
            }
        };

        let vm = Vm::new(chunk);
        match vm.interpret() {
            Ok(_) => {}
            Err(err) => match err {
                InterpretError::RuntimeError(msg) => {
                    println!("{}", msg);
                }
            },
        }
    }
}

fn main() {
    repl();

    // let program = "2 * 2 * 2";
    // let scanner = Scanner::new(program);
    // let parser = Parser::new(scanner);
    // let chunk = parser.compile().unwrap();
    // let interpret_result = Vm::new(chunk).interpret();
    // println!("{:?}", interpret_result);
}
