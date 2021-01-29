use std::{iter::Peekable, slice::Iter, todo};

use crate::{ast::{Binary, Expr, ExpressionStatement, Literal, PrintStatement, Statement, Unary, UnaryOperator, Variable}, error::{LoxError, LoxErrorKind}, tokens::{Token, TokenType}};
use crate::ast::{BinaryOperator};


pub(crate) struct Parser {

}

impl Parser {

    // program -> statement EOF ;
    pub fn parse<'a>(&self, tokens: &'a Vec<Token>) -> Result<Statement<'a>, LoxError> {
        let mut tokens = tokens.iter().peekable();
        self.statement(&mut tokens)
    }

    // statement -> exprStatement
    // | printStatement ;
    fn statement<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
        match &tokens.peek().unwrap().token_type {
            TokenType::Print => {
                self.print_statement(tokens)
            },
            _ => {
                // if the next token doesn't like any other statement, assume its an expr statement
                self.expression_statement(tokens)
            }
        }
    }

    // printStatement -> "print" expression ";" ;
    fn print_statement<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
        let token = tokens.next().unwrap(); // "print" token
        let value = self.expression(tokens)?;
        match &tokens.peek().unwrap().token_type {
            TokenType::Semicolon => {
                tokens.next(); // consume ";"
            },
            _ => {
              return Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "expected ';' after statement"})  
            }
        };
        Ok(Statement::PrintStatement(PrintStatement {token, value}))
    }

    // exprStatement -> expression ";" ;
    fn expression_statement<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
        let expr = self.expression(tokens)?;
        match &tokens.peek().unwrap().token_type {
            TokenType::Semicolon => {
                tokens.next(); // consume ";"
            },
            _ => {
              return Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "expected ';' after statement"})  
            }
        };
        Ok(Statement::ExpressionStatement(ExpressionStatement {expression: expr}))
    }

    // expression -> equality
    fn expression<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
        self.equality(tokens)
    }

    // equality -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
        let mut expr = self.comparison(tokens)?;
        loop {
            let operator;
            let token;
            match &tokens.peek().unwrap().token_type {
                TokenType::BangEqual => {
                    token = tokens.next().unwrap();
                    operator = BinaryOperator::BangEqual;
                },
                TokenType::EqualEqual => {
                    token = tokens.next().unwrap();
                    operator = BinaryOperator::EqualEqual;
                }
                _ => break
            }
            let right = self.comparison(tokens)?;
            expr = Expr::Binary(Binary {token: token, operator: operator, left: Box::new(expr), right: Box::new(right)});
        };
        Ok(expr)
    }

    // comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison<'a>(& self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
        let mut expr = self.term(tokens)?;
        loop {
            let operator;
            let token;
            match &tokens.peek().unwrap().token_type {
                TokenType::Greater => {
                    token = tokens.next().unwrap();
                    operator = BinaryOperator::Greater;
                },
                TokenType::GreaterEqual => {
                    token = tokens.next().unwrap();
                    operator = BinaryOperator::GreaterEqual;
                },
                TokenType::Less => {
                    token = tokens.next().unwrap();
                    operator = BinaryOperator::Less;
                },
                TokenType::LessEqual => {
                    token = tokens.next().unwrap();
                    operator = BinaryOperator::LessEqual;
                },
                _ => break
            }
            let right = self.term(tokens)?;
            expr = Expr::Binary(Binary {token: token, operator: operator, left: Box::new(expr), right: Box::new(right)});
        };
        Ok(expr)
    }

    // term -> factor ( ( "-" | "+") factor )* ;
    fn term<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
        let mut expr = self.factor(tokens)?;
        loop {
            let operator;
            let token;
            match &tokens.peek().unwrap().token_type {
                TokenType::Minus => {
                    token = tokens.next().unwrap();
                    operator = BinaryOperator::Minus;
                },
                TokenType::Plus => {
                    token = tokens.next().unwrap();
                    operator = BinaryOperator::Plus;
                },
                _ => break
            }
            let right = self.factor(tokens)?;
            expr = Expr::Binary(Binary {token: token, operator: operator, left: Box::new(expr), right: Box::new(right)});
        }
        Ok(expr)
    }

    // factor -> unary ( ( "/" | "*") unary )* ;
    fn factor<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError>{
        let mut expr = self.unary(tokens)?;
        loop {
            let operator;
            let token;
            match &tokens.peek().unwrap().token_type {
                TokenType::Slash => {
                    token = tokens.next().unwrap();
                    operator = BinaryOperator::Slash;
                },
                TokenType::Star => {
                    token = tokens.next().unwrap();
                    operator = BinaryOperator::Star;
                },
                _ => break
            }
            let right = self.unary(tokens)?;
            expr = Expr::Binary(Binary {token: token, operator: operator, left: Box::new(expr), right: Box::new(right)});
        }
        Ok(expr)
    }

    // unary -> ( "!" | "-" ) unary
    //       | primary ;
    fn unary<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
        match &tokens.peek().unwrap().token_type {
            TokenType::Bang => {
                let token = tokens.next().unwrap();
                let operator = UnaryOperator::Bang;
                let right = self.unary(tokens)?;
                Ok(Expr::Unary(Unary {operator: operator, token: token, right: Box::new(right)}))
            },
            TokenType::Minus => {
                let token = tokens.next().unwrap();
                let operator = UnaryOperator::Minus;
                let right = self.unary(tokens)?;
                Ok(Expr::Unary(Unary {operator: operator, token: token, right: Box::new(right)}))
            }
            _ => {
                self.primary(tokens)
            }
        }
    }

    // primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
        match &tokens.peek().unwrap().token_type {
            TokenType::False | TokenType::True | TokenType::Number | TokenType::String | TokenType::Nil => {
                let token = tokens.next().unwrap();
                let value = token.literal.clone().unwrap();
                Ok(Expr::Literal(Literal { token, value }))
            },
            TokenType::Identifier => {
                Ok(Expr::Variable(Variable { token: tokens.next().unwrap() }))
            },
            TokenType::LeftParen => {
                tokens.next(); // consume '('
                let expr = self.expression(tokens)?;
                match &tokens.peek().unwrap().token_type {
                    TokenType::RightParen => {
                        tokens.next() // consume matching ')'
                    },
                    _ => {
                        return Err(LoxError {kind: LoxErrorKind::ScannerError, message: "expected ')' after expression"})
                    }
                };
                Ok(expr)
            }
            _ => {
                Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "invalid syntax"})
            }
        }
    }
}