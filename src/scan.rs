use std::iter::Peekable;
use std::str::Chars;

use crate::{error::LoxError, tokens::Token};
use crate::tokens::TokenType;

pub(crate) struct Scanner<'a> {
    source: &'a String,
    pub tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: i32
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Scanner<'a> {
        Scanner { source: source, tokens: Vec::<Token>::new(),
        start: 0, current: 0, line: 1 }
    }

    pub fn scan(&mut self) -> Result<(), LoxError> {
        let mut chars = self.source.chars().peekable();
        
        loop {
            if chars.peek().is_none() {
                break;
            }
            self.scan_token(&mut chars)?;
            self.start = self.current;
        }
        self.tokens.push(Token {lexeme: "", line: self.line, literal: None, token_type: TokenType::EOF});
        Ok(())
    }

    fn scan_token(&mut self, chars: &mut Peekable<Chars<'_>>) -> Result<(), LoxError> {
        // we can unwrap here, since we peeked before this and know that the result is Some not None
        let s = self.advance(chars).unwrap();
        match s {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let tt = if self.match_next('=', chars) { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(tt);
            },
            '=' => {
                let tt = if self.match_next('=', chars) { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(tt);
            },
            '<' => {
                let tt = if self.match_next('=', chars) { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(tt)
            },
            '>' => {
                let tt = if self.match_next('=', chars) { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(tt);
            },
            '\n' => {
                self.line += 1;
            }

            _ => return Err(LoxError { kind: crate::error::LoxErrorKind::ScannerError, message: "scanner error" })
        }
        Ok(())
    }

    fn advance(&mut self, chars: &mut Peekable<Chars<'_>>) -> Option<char> {
        self.current += 1;
        chars.next()
    }

    fn match_next(&mut self, expected: char, chars: &mut Peekable<Chars<'_>>) -> bool {
        if let Some(peeked) = chars.peek() {
            if peeked == &expected {
                self.advance(chars);
                return true;
            }
        }
        return false;
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token {token_type: token_type, lexeme: lexeme, literal: None, line: self.line};
        self.tokens.push(token);
    }
}