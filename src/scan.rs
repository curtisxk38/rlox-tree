use std::iter::Peekable;
use std::str::Chars;

use crate::{error::{LoxError, LoxErrorKind}, tokens::{LiteralValue, Token}};
use crate::tokens::TokenType;

pub(crate) struct Scanner {
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
    next_id: u32,
}

impl<'c> Scanner {
    pub fn new() -> Scanner {
        Scanner { tokens: Vec::<Token>::new(), start: 0, current: 0, line: 1, next_id: 0 }
    }

    pub fn scan(&mut self, source: &'c String) -> Result<(), LoxError> {
        // needed since scan can be called more than once for a given Scanner
        //  if the interpreter is running as a REPL
        // however we purposefully do not re-set next_id since IDs should be unique for ever token
        self.tokens = Vec::new(); 
        self.start = 0;
        self.current = 0;
        self.line = 1;

        let mut chars = source.chars().peekable();
        
        loop {
            if chars.peek().is_none() {
                break;
            }
            self.scan_token(&mut chars, source)?;
            self.start = self.current;
        }
        self.add_token(TokenType::EOF, "".to_owned(), None);
        Ok(())
    }

    fn scan_token(&mut self, chars: &mut Peekable<Chars<'_>>, source: &'c String) -> Result<(), LoxError> {
        // we can unwrap here, since we peeked before this and know that the result is Some not None
        let s = self.advance(chars).unwrap();
        match s {
            '(' => self.add_simple_token(TokenType::LeftParen, source),
            ')' => self.add_simple_token(TokenType::RightParen, source),
            '{' => self.add_simple_token(TokenType::LeftBrace, source),
            '}' => self.add_simple_token(TokenType::RightBrace, source),
            ',' => self.add_simple_token(TokenType::Comma, source),
            '.' => self.add_simple_token(TokenType::Dot, source),
            '-' => self.add_simple_token(TokenType::Minus, source),
            '+' => self.add_simple_token(TokenType::Plus, source),
            ';' => self.add_simple_token(TokenType::Semicolon, source),
            '*' => self.add_simple_token(TokenType::Star, source),
            '!' => {
                let tt = if self.match_next('=', chars) { TokenType::BangEqual } else { TokenType::Bang };
                self.add_simple_token(tt, source);
            },
            '=' => {
                let tt = if self.match_next('=', chars) { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_simple_token(tt, source);
            },
            '<' => {
                let tt = if self.match_next('=', chars) { TokenType::LessEqual } else { TokenType::Less };
                self.add_simple_token(tt, source)
            },
            '>' => {
                let tt = if self.match_next('=', chars) { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_simple_token(tt, source);
            },
            '/' => {
                if self.match_next('/', chars) {
                    // if you see '//' keep consuming characters until '\n'
                    loop {
                        if let Some(c) = chars.peek() {
                            if c == &'\n' {
                                break;
                            } else {
                                self.advance(chars);
                            }
                        }
                    }
                } else {
                    self.add_simple_token(TokenType::Slash, source);
                }
            },
            ' ' | '\t' | '\r' => {},
            '\n' => {
                self.line += 1;
            },
            '"' => {
                return self.scan_string(chars, source);
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                return self.scan_number(chars, source)
            }
            _ => {
                if s.is_alphabetic() {
                    return self.scan_alphabetic(chars, source)
                } else {
                    return Err(LoxError { kind: crate::error::LoxErrorKind::ScannerError, message: "unexpected character" })
                }
            }
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

    fn add_simple_token(&mut self, token_type: TokenType, source: &'c String) {
        let lexeme = &source[self.start..self.current];
        self.add_token(token_type, lexeme.to_owned(), None);
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String, literal: Option<LiteralValue>) {
        let t = Token {token_type, lexeme, literal, line: self.line, id: self.next_id};
        self.next_id += 1;
        self.tokens.push(t);
    }

    fn scan_string(&mut self, chars: &mut Peekable<Chars<'_>>, source: &'c String) -> Result<(), LoxError> {
        loop {
            match self.advance(chars) {
                Some(char) => {
                    if char == '"' {
                        // reached end of string literal
                        break;
                    } else {
                        if char == '\n' {
                            self.line += 1;
                        }
                    }
                },
                None => {
                    return Err(LoxError { kind: LoxErrorKind::ScannerError, message: "untermianted string "});
                }
            }
        }
        let lexeme = &source[self.start..self.current];
        // the lexeme includes the literal ", but we don't want the String to include this
        //  so we don't include the first and last chars of the lexeme
        let literal = String::from(&source[self.start+1..self.current-1]);
        self.add_token(TokenType::String, lexeme.to_owned(), Some(LiteralValue::StringValue(literal)));
        Ok(())
    }

    fn scan_number(&mut self, chars: &mut Peekable<Chars<'_>>, source: &'c String) -> Result<(), LoxError> {
        loop {
            if let Some(next) = chars.peek() {
                match next {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        self.advance(chars);
                    },
                    '.' => {
                        let mut peek_more = chars.clone();
                        peek_more.next(); // consume the '.' in this interator
                        if let Some(after_dot) = peek_more.next() {
                            match after_dot {
                                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                                    self.advance(chars); // this consumes the '.'
                                    // now keep consuming numbers as you see them
                                    loop {
                                        if let Some(number_after_dot) = chars.peek() {
                                            match number_after_dot {
                                                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => { 
                                                    self.advance(chars);
                                                },
                                                _ => break
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                },
                                _ => break
                            }
                        } else {
                            break;
                        }
                    },
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        let lexeme = &source[self.start..self.current];
        let number_conversion = lexeme.parse::<f64>();
        if let Ok(number) = number_conversion {
            let literal = Some(LiteralValue::NumberValue(number));
            self.add_token(TokenType::Number, lexeme.to_owned(), literal);
            Ok(())
        } else {
            Err(LoxError { kind: LoxErrorKind::ScannerError, message: "unable to parse float"})
        }
    }

    fn scan_alphabetic(&mut self, chars: &mut Peekable<Chars<'_>>, source: &'c String) -> Result<(), LoxError> {
        loop {
            if let Some(possible_alphabetic) = chars.peek() {
                if possible_alphabetic.is_alphanumeric() {
                    self.advance(chars);
                } else {
                    break;
                }
            } else {
                break;
            }
        };
        let lexeme = &source[self.start..self.current];
        let token_type = match lexeme {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier
        };
        let literal = match &token_type {
            TokenType::False => Some(LiteralValue::BooleanValue(false)),
            TokenType::True => Some(LiteralValue::BooleanValue(true)),
            TokenType::Nil => Some(LiteralValue::NilValue),
            _ => None
        };
        self.add_token(token_type, lexeme.to_owned(), literal);
        Ok(())
    }
}