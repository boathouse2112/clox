mod parser;
mod scanner;
mod stack;
mod value;
mod vm;

use crate::scanner::{Scanner, TokenType};
use std::{env, io};

// fn repl() {
//     let stdin = io::stdin();
//     let mut line = String::new();
//     loop {
//         line.clear();
//         _ = stdin.read_line(&mut line).or_else(|| {
//             panic!("Invalid UTF-8 input.");
//         });
//
//         let vm = Vm::new(chunk);
//         vm.interpret(&line);
//     }
// }

fn main() {
    // let args: Vec<String> = env::args().collect();

    // if args.len() == 1 {
    //     repl(vm);
    // } else if args.len() == 2 {
    //     run_file(&args[1]);
    // } else {
    //     eprintln!("Usage: clox <file>");
    // }

    let program = "(){},.-+;/*";
    let mut scanner = Scanner::new(program);
    let mut tokens = Vec::new();
    loop {
        let token = scanner.scan();
        tokens.push(token);
        if token.token_type == TokenType::Eof {
            break;
        }
    }
    println!("{:?}", tokens);
}
