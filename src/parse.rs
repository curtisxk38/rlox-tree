use std::{iter::Peekable, slice::Iter};

use crate::{ast::{Assignent, Binary, Expr, ExpressionStatement, Grouping, Literal, PrintStatement, Statement, Unary, UnaryOperator, VarDeclStatement, Variable}, error::{LoxError, LoxErrorKind}, tokens::{Token, TokenType}};
use crate::ast::{BinaryOperator};


pub(crate) struct Parser {

}

impl Parser {

    // program -> statement* EOF ;
    pub fn parse<'a>(&self, tokens: &'a Vec<Token>) -> Result<Vec<Statement<'a>>, Vec<LoxError>> {
        let mut tokens = tokens.iter().peekable();
        let mut statements: Vec<Statement> = Vec::new();
        let mut errors: Vec<LoxError> = Vec::new();
        
        loop {
            match tokens.peek() {
                Some(token) => {
                    match token.token_type {
                        TokenType::EOF => {
                            break
                        },
                        _ => {
                            let result = self.declaration(&mut tokens);
                            match result {
                                Ok(s) => {
                                    statements.push(s)
                                },
                                Err(e) => {
                                    errors.push(e);
                                    self.synchronize(&mut tokens);
                                }
                            }
                        }
                    }
                }
                None => {
                    break;
                }
            }
            
        }

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(statements)
        }
    }

    fn synchronize<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) {
        let mut next = tokens.next();

        loop {
            match next {
                Some(token) => {
                    // if we just consumed a semicolon,
                    // we're synchronized and ready to parse the next statement
                    match token.token_type {
                        TokenType::Semicolon => break,
                        _ => {}
                    };
        
                    match tokens.peek() {
                        // if the next token in the list is one of the below
                        // we are ready to start parsing the next statement,
                        // since these token types all are used to start statements
                        Some(peeked) => {
                            match peeked.token_type {
                                TokenType::Class => break,
                                TokenType::Fun => break,
                                TokenType::Var => break,
                                TokenType::For => break,
                                TokenType::If => break,
                                TokenType::While => break,
                                TokenType::Print => break,
                                TokenType::Return => break,
                                _ => {}
                            }
                        },
                        None => break
                    };
        
                    next = tokens.next();
                },
                None => {
                    break;
                }
            }
        }
    }

    // declaration -> varDecl | statement ;
    fn declaration<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
        match &tokens.peek().unwrap().token_type {
            TokenType::Var => {
                self.var_declaration(tokens)
            },
            _ => self.statement(tokens)
        }
    }

    fn var_declaration<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
        tokens.next(); // consume 'var'
        let token;
        match &tokens.peek().unwrap().token_type {
            TokenType::Identifier => token = tokens.next().unwrap(),
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "expected identifier"})
            }
        };

        let initializer;
        match &tokens.peek().unwrap().token_type {
            TokenType::Equal => {
                tokens.next(); // consume '='
                initializer = Some(self.expression(tokens)?);
            },
            _ => {
                initializer = None;
            }
        };

        match &tokens.peek().unwrap().token_type {
            TokenType::Semicolon => {
                tokens.next(); // consume ";"
            },
            _ => {
              return Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "expected ';' variable declaration"})  
            }
        };
        Ok(Statement::VarDeclStatement(VarDeclStatement {token, initializer}))
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

    // expression -> assignment ;
    fn expression<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
        self.assignment(tokens)
    }

    // assignment -> IDENTIFIER "=" assignment | equality ;
    fn assignment<'a>(&self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
        let expr = self.equality(tokens)?;

        match &tokens.peek().unwrap().token_type {
            TokenType::Equal => {
                tokens.next().unwrap(); // consume "="
                let value = self.assignment(tokens)?;
                match &expr {
                    Expr::Variable(v) => {
                        let name = v.token;
                        return Ok(Expr::Assignent(Assignent {token: name, value: Box::new(value)}));
                    },
                    _ => {}
                };
                Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "invalid assignment target"})
            },
            _ => {
                Ok(expr)
            }
        }
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
                Ok(Expr::Grouping(Grouping {expr: Box::new(expr)}))
            }
            _ => {
                Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "invalid syntax"})
            }
        }
    }
}