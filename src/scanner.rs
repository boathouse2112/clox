use peekmore::{PeekMore, PeekMoreIterator};
use std::str::Chars;

use TokenType::*;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fn,
    If,
    Let,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    While,
    // Misc.
    Error,
    Eof,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Token<'text> {
    pub token_type: TokenType,
    pub text: &'text str,
}

pub struct Scanner<'text> {
    text: &'text str,
    iter: PeekMoreIterator<Chars<'text>>,
    current: usize,
}

impl<'text> Scanner<'text> {
    pub fn new(text: &'text str) -> Self {
        let iter = text.chars().peekmore();
        Self {
            text,
            iter,
            current: 0,
        }
    }

    pub fn scan(&mut self) -> Token<'text> {
        self.skip_whitespace();
        let start = self.current;
        let Some(char) = self.advance() else {
            return self.make_token(Eof, start);
        };

        if char.is_alphabetic() {
            return self.identifier(start);
        } else if char.is_ascii_digit() {
            return self.number(start);
        }

        match char {
            '(' => self.make_token(LeftParen, start),
            ')' => self.make_token(RightParen, start),
            '{' => self.make_token(LeftBrace, start),
            '}' => self.make_token(RightBrace, start),
            ';' => self.make_token(Semicolon, start),
            ',' => self.make_token(Comma, start),
            '.' => self.make_token(Dot, start),
            '-' => self.make_token(Minus, start),
            '+' => self.make_token(Plus, start),
            '/' => self.make_token(Slash, start),
            '*' => self.make_token(Star, start),
            '!' => {
                let token_type = if self.match_char('=') {
                    BangEqual
                } else {
                    Bang
                };
                self.make_token(token_type, start)
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.make_token(token_type, start)
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    LessEqual
                } else {
                    Less
                };
                self.make_token(token_type, start)
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.make_token(token_type, start)
            }
            '"' => self.string(start),
            unexpected => panic!("Unexpected char: {}", unexpected),
        }
    }

    /// Advances forward one character.
    /// If we reach the end of the iter, returns None.
    fn advance(&mut self) -> Option<char> {
        match self.iter.next() {
            None => None,
            char => {
                self.current += 1;
                char
            }
        }
    }

    /// Peek the next char and its index.
    fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    fn peek_next(&mut self) -> Option<&char> {
        self.iter.peek_nth(2)
    }

    /// If self.peek() == char, consume it and return true.
    /// Else return false.
    fn match_char(&mut self, char: char) -> bool {
        if self.peek().is_some_and(|&next| next == char) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_rest_of_line(&mut self) {
        while self.peek().is_some_and(|&next| next != '\n') {
            self.advance();
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                None => return,
                Some(next) => match next {
                    ' ' | '\r' | '\t' | '\n' => {
                        self.advance();
                    } // Consume it
                    '/' => {
                        // Check if it's a comment
                        if self.peek_next().is_some_and(|&char| char == '/') {
                            self.skip_rest_of_line();
                        }
                        return;
                    }
                    _ => return,
                },
            }
        }
    }

    /// Consume a keyword or an identifier.
    fn identifier(&mut self, start: usize) -> Token<'text> {
        while self
            .peek()
            .is_some_and(|next| next.is_alphabetic() || next.is_ascii_digit())
        {
            self.advance();
        }

        let chars = &self.text[start..self.current];
        match chars {
            "and" => self.make_token(And, start),
            "class" => self.make_token(Class, start),
            "else" => self.make_token(Else, start),
            "false" => self.make_token(False, start),
            "for" => self.make_token(For, start),
            "fn" => self.make_token(Fn, start),
            "if" => self.make_token(If, start),
            "let" => self.make_token(Let, start),
            "nil" => self.make_token(Nil, start),
            "or" => self.make_token(Or, start),
            "print" => self.make_token(Print, start),
            "return" => self.make_token(Return, start),
            "super" => self.make_token(Super, start),
            "this" => self.make_token(This, start),
            "true" => self.make_token(True, start),
            "while" => self.make_token(While, start),
            _ => self.make_token(Identifier, start),
        }
    }

    fn number(&mut self, start: usize) -> Token<'text> {
        // Consume digits, then maybe ('.' and digits)
        while self.peek().is_some_and(|next| next.is_ascii_digit()) {
            self.advance();
        }
        if self.peek().is_some_and(|&next| next == '.')
            && self.peek_next().is_some_and(|char| char.is_ascii_digit())
        {
            self.advance();
        }
        while self.peek().is_some_and(|next| next.is_ascii_digit()) {
            self.advance();
        }

        self.make_token(Number, start)
    }

    fn string(&mut self, start: usize) -> Token<'text> {
        loop {
            let Some(char) = self.advance() else {
                return Self::error_token("Unterminated string");
            };
            if char == '"' {
                return self.make_token(String, start);
            }
        }
    }

    fn make_token(&self, token_type: TokenType, start: usize) -> Token<'text> {
        Token {
            token_type,
            text: &self.text[start..self.current],
        }
    }

    fn error_token(text: &'static str) -> Token<'text> {
        Token {
            token_type: Error,
            text,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::{Scanner, Token, TokenType};

    use TokenType::*;

    fn scan_tokens(program: &str) -> Vec<Token> {
        let mut scanner = Scanner::new(program);
        let mut tokens = Vec::new();
        loop {
            let token = scanner.scan();
            tokens.push(token);
            if token.token_type == Eof {
                break;
            }
        }
        tokens
    }

    fn tok(token_type: TokenType, text: &str) -> Token {
        Token { token_type, text }
    }

    #[test]
    fn empty() {
        let program = "";
        let tokens = scan_tokens(program);
        let expected = vec![tok(Eof, "")];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn single_character_tokens() {
        let program = "(){},.-+;/*";
        let tokens = scan_tokens(program);
        let expected = vec![
            tok(LeftParen, "("),
            tok(RightParen, ")"),
            tok(LeftBrace, "{"),
            tok(RightBrace, "}"),
            tok(Comma, ","),
            tok(Dot, "."),
            tok(Minus, "-"),
            tok(Plus, "+"),
            tok(Semicolon, ";"),
            tok(Slash, "/"),
            tok(Star, "*"),
            tok(Eof, ""),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn one_or_two_character_tokens() {
        let program = "! != = == > >= < <=";
        let tokens = scan_tokens(program);
        let expected = vec![
            tok(Bang, "!"),
            tok(BangEqual, "!="),
            tok(Equal, "="),
            tok(EqualEqual, "=="),
            tok(Greater, ">"),
            tok(GreaterEqual, ">="),
            tok(Less, "<"),
            tok(LessEqual, "<="),
            tok(Eof, ""),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn identifiers() {
        let program = "mark is making an interpreter";
        let tokens = scan_tokens(program);
        let expected = vec![
            tok(Identifier, "mark"),
            tok(Identifier, "is"),
            tok(Identifier, "making"),
            tok(Identifier, "an"),
            tok(Identifier, "interpreter"),
            tok(Eof, ""),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn empty_string() {
        let program = "\"\"";
        let tokens = scan_tokens(program);
        let expected = vec![tok(String, "\"\""), tok(Eof, "")];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn non_empty_string() {
        let program = "\"keywords: if else nil\"";
        let tokens = scan_tokens(program);
        let expected = vec![tok(String, "\"keywords: if else nil\""), tok(Eof, "")];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn integer() {
        let program = "01234";
        let tokens = scan_tokens(program);
        let expected = vec![tok(Number, "01234"), tok(Eof, "")];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn decimal() {
        let program = "0123.456";
        let tokens = scan_tokens(program);
        let expected = vec![tok(Number, "0123.456"), tok(Eof, "")];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn keywords() {
        let program =
            "and class else false for fn if let nil or print return super this true while";
        let tokens = scan_tokens(program);
        let expected = vec![
            tok(And, "and"),
            tok(Class, "class"),
            tok(Else, "else"),
            tok(False, "false"),
            tok(For, "for"),
            tok(Fn, "fn"),
            tok(If, "if"),
            tok(Let, "let"),
            tok(Nil, "nil"),
            tok(Or, "or"),
            tok(Print, "print"),
            tok(Return, "return"),
            tok(Super, "super"),
            tok(This, "this"),
            tok(True, "true"),
            tok(While, "while"),
            tok(Eof, ""),
        ];
        assert_eq!(tokens, expected);
    }
}
