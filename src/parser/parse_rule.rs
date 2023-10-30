// use crate::parser::{ParseResult, Parser, Precedence};
// use crate::scanner::TokenType;
// use lazy_static::lazy_static;
// use std::collections::HashMap;
//
// pub struct ParseRule {
//     prefix: Option<ParseFn>,
//     infix: Option<ParseFn>,
//     precedence: Precedence,
// }
//
// impl ParseRule {
//     pub fn new(prefix: Option<ParseFn>, infix: Option<ParseFn>, precedence: Precedence) -> Self {
//         Self {
//             prefix,
//             infix,
//             precedence,
//         }
//     }
// }
//
// pub const fn parse_rule(token_type: TokenType) -> ParseRule {
//     match token_type {
//         TokenType::LeftParen => ParseRule::new(Some(Parser::grouping), None, Precedence::None),
//         TokenType::RightParen => ParseRule::new(None, None, Precedence::None),
//         TokenType::LeftBrace => ParseRule::new(None, None, Precedence::None),
//         TokenType::RightBrace => ParseRule::new(None, None, Precedence::None),
//         TokenType::Comma => ParseRule::new(None, None, Precedence::None),
//         TokenType::Dot => ParseRule::new(None, None, Precedence::None),
//         TokenType::Minus => ParseRule::new(Parser::unary, Parser::binary, Precedence::Term),
//         TokenType::Plus => ParseRule::new(None, Parser::binary, Precedence::Term),
//         TokenType::Semicolon => ParseRule::new(None, None, Precedence::None),
//         TokenType::Slash => ParseRule::new(None, Parser::binary, Precedence::Factor),
//         TokenType::Star => ParseRule::new(None, Parser::binary, Precedence::Factor),
//         TokenType::Bang => ParseRule::new(None, None, Precedence::None),
//         TokenType::BangEqual => ParseRule::new(None, None, Precedence::None),
//         TokenType::Equal => ParseRule::new(None, None, Precedence::None),
//         TokenType::EqualEqual => ParseRule::new(None, None, Precedence::None),
//         TokenType::Greater => ParseRule::new(None, None, Precedence::None),
//         TokenType::GreaterEqual => ParseRule::new(None, None, Precedence::None),
//         TokenType::Less => ParseRule::new(None, None, Precedence::None),
//         TokenType::LessEqual => ParseRule::new(None, None, Precedence::None),
//         TokenType::Identifier => ParseRule::new(None, None, Precedence::None),
//         TokenType::String => ParseRule::new(None, None, Precedence::None),
//         TokenType::Number => ParseRule::new(Parser::number, None, Precedence::None),
//         TokenType::And => ParseRule::new(None, None, Precedence::None),
//         TokenType::Class => ParseRule::new(None, None, Precedence::None),
//         TokenType::Else => ParseRule::new(None, None, Precedence::None),
//         TokenType::False => ParseRule::new(Parser::literal, None, Precedence::None),
//         TokenType::For => ParseRule::new(None, None, Precedence::None),
//         TokenType::Fn => ParseRule::new(None, None, Precedence::None),
//         TokenType::If => ParseRule::new(None, None, Precedence::None),
//         TokenType::Let => ParseRule::new(None, None, Precedence::None),
//         TokenType::Nil => ParseRule::new(Parser::literal, None, Precedence::None),
//         TokenType::Or => ParseRule::new(None, None, Precedence::None),
//         TokenType::Print => ParseRule::new(None, None, Precedence::None),
//         TokenType::Return => ParseRule::new(None, None, Precedence::None),
//         TokenType::Super => ParseRule::new(None, None, Precedence::None),
//         TokenType::This => ParseRule::new(None, None, Precedence::None),
//         TokenType::True => ParseRule::new(Parser::literal, None, Precedence::None),
//         TokenType::While => ParseRule::new(None, None, Precedence::None),
//         TokenType::Error => ParseRule::new(None, None, Precedence::None),
//         TokenType::Eof => ParseRule::new(None, None, Precedence::None),
//     }
// }
