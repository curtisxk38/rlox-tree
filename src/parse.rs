use std::{iter::Peekable, slice::Iter};

use crate::{ast::{Assignment, Binary, BlockStatement, Call, ClassDeclStatement, Expr, ExpressionStatement, FunDeclStatement, Get, Grouping, IfStatement, Literal, Logical, LogicalOperator, PrintStatement, ReturnStatement, Set, Statement, Super, This, Unary, UnaryOperator, VarDeclStatement, Variable, WhileStatement}, error::{LoxError, LoxErrorKind}, tokens::{LiteralValue, Token, TokenType}};
use crate::ast::{BinaryOperator};


const MAX_PARAMETERS: usize = 255;

pub(crate) struct Parser {
    pub errors: Vec<LoxError>,
}

enum FunctionKind {
    Function,
    Method,
}

impl Parser {
    

    pub fn new() -> Parser {
        Parser { errors: Vec::new() }
    }

    // program -> statement* EOF ;
    pub fn parse(&mut self, tokens: & Vec<Token>) -> Result<Vec<Statement>, ()> {
        let mut tokens = tokens.iter().peekable();
        let mut statements: Vec<Statement> = Vec::new();
        
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
                                    self.errors.push(e);
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

        if self.errors.len() > 0 {
            Err(())
        } else {
            Ok(statements)
        }
    }

    fn synchronize(&mut self, tokens: &mut Peekable<Iter<Token>>) {
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

    // declaration -> classDecl | funDecl | varDecl | statement ;
    fn declaration(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Statement, LoxError> {
        match &tokens.peek().unwrap().token_type {
            TokenType::Class => {
                self.class_declaration(tokens)
            },
            TokenType::Var => {
                self.var_declaration(tokens)
            },
            TokenType::Fun => {
                self.fun_declaration(tokens)
            }
            _ => self.statement(tokens)
        }
    }

    // classDecl -> "class" IDENTIFIER ( "<" IDENTIFIER )? "{" function* "}" ;
    // Unlike function declarations, methods don’t have a leading fun keyword
    fn class_declaration(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Statement, LoxError> {
        tokens.next(); // consume 'class'
        let name;
        let superclass;
        match &tokens.peek().unwrap().token_type {
            TokenType::Identifier => name = tokens.next().unwrap().to_owned(),
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "Expected class name"})
            }
        };
        match &tokens.peek().unwrap().token_type {
            TokenType::Less => {
                tokens.next(); // consume '<'
                match &tokens.peek().unwrap().token_type {
                    TokenType::Identifier => {
                        superclass = Some(Variable{ token: tokens.next().unwrap().to_owned() });
                    },
                    _ => {
                        return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "Expected super class name"})
                    }
                }
            },
            _ => {
                superclass = None;
            }
        };
        match &tokens.peek().unwrap().token_type {
            TokenType::LeftBrace => tokens.next(), // consume '{'
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "Expected { after class declaration"})
            }
        };
        let mut methods = Vec::new();
        loop {
            match &tokens.peek().unwrap().token_type {
                TokenType::RightBrace => {
                    tokens.next(); // consume '}'
                    break;
                },
                TokenType::EOF => {
                    return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "reached EOF while parsing, expected '}'"})
                }
                _ => {}
            };
            methods.push(self.function(tokens, FunctionKind::Method)?);
        };
        Ok(Statement::ClassDeclStatement(ClassDeclStatement {name, methods, superclass}))
    }

    // funDecl -> "fun" function ;
    fn fun_declaration(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Statement, LoxError> {
        tokens.next(); // consume 'fun'
        Ok(Statement::FunDeclStatement(self.function(tokens, FunctionKind::Function)?))
    }

    // function -> IDENTIFIER "(" parameters? ")" blockStatement ;
    fn function(&mut self, tokens: &mut Peekable<Iter<Token>>, kind: FunctionKind) -> Result<FunDeclStatement, LoxError> {
        let name;
        match &tokens.peek().unwrap().token_type {
            TokenType::Identifier => name = tokens.next().unwrap().to_owned(),
            _ => {
                let message = match kind {
                    FunctionKind::Function => { "expected function name"}
                    FunctionKind::Method => { "expected method name" }
                };
                return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message});
            }
        };

        match &tokens.peek().unwrap().token_type {
            TokenType::LeftParen => {
                tokens.next(); // consume "("
                let mut parameters = Vec::new();
                match &tokens.peek().unwrap().token_type {
                    TokenType::RightParen => {
                        // no parameters
                    },
                    _ => {
                        loop {
                            if parameters.len() > MAX_PARAMETERS {
                                // no need to return the Error
                                // that would mean the parser is in a bad state and needs to synchronize
                                // but we don't need to do that for this type of error
                                self.errors.push(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "can't have > 255 arguments to a function call"})
                            }
                            match &tokens.peek().unwrap().token_type {
                                TokenType::Identifier => {
                                    parameters.push(tokens.next().unwrap().to_owned());
                                },
                                _ => {
                                    return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected identifier"});
                                }
                            }

                            match &tokens.peek().unwrap().token_type {
                                TokenType::Comma => {
                                    tokens.next(); // consume ','
                                },
                                _ => {
                                    break;
                                }
                            }
                        }
                    }
                }

                match &tokens.peek().unwrap().token_type {
                    TokenType::RightParen => {
                        tokens.next(); // consume ")"
                    },
                    _ => {
                        return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected ')' after parameters"})
                    }
                }

                let body;
                match &tokens.peek().unwrap().token_type {
                    TokenType::LeftBrace => {
                        body = self.block(tokens)?;
                    },
                    _ => {
                        let message = match kind {
                            FunctionKind::Function => { "expected '{' afer function body"}
                            FunctionKind::Method => { "expected '{' after method body" }
                        };
                        return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message})
                    }
                };


                Ok(FunDeclStatement {name, body, parameters})
            },
            _ => {
                let message = match kind {
                    FunctionKind::Function => { "expected '(' afer function name"}
                    FunctionKind::Method => { "expected '(' after method name" }
                };
                Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message})
            }
        }
    }

    fn var_declaration(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Statement, LoxError> {
        tokens.next(); // consume 'var'
        let token;
        match &tokens.peek().unwrap().token_type {
            TokenType::Identifier => token = tokens.next().unwrap().to_owned(),
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected identifier"})
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
              return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected ';' variable declaration"})  
            }
        };
        Ok(Statement::VarDeclStatement(VarDeclStatement {token, initializer}))
    }

    // statement -> exprStatement
    // | printStatement 
    // | blockStatement 
    // | ifStatement
    // | whileStatement 
    // | forStatement
    // | returnStatement ;
    fn statement(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Statement, LoxError> {
        match &tokens.peek().unwrap().token_type {
            TokenType::Print => {
                self.print_statement(tokens)
            },
            TokenType::LeftBrace => {
                Ok(Statement::BlockStatement(self.block_statement(tokens)?))
            },
            TokenType::If => {
                self.if_statement(tokens)
            },
            TokenType::While => {
                self.while_statement(tokens)
            },
            TokenType::For => {
                self.for_statement(tokens)
            },
            TokenType::Return => {
                self.return_statement(tokens)
            }
            _ => {
                // if the next token doesn't like any other statement, assume its an expr statement
                self.expression_statement(tokens)
            }
        }
    }

    // printStatement -> "print" expression ";" ;
    fn print_statement(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Statement, LoxError> {
        let token = tokens.next().unwrap().to_owned(); // "print" token
        let value = self.expression(tokens)?;
        match &tokens.peek().unwrap().token_type {
            TokenType::Semicolon => {
                tokens.next(); // consume ";"
            },
            _ => {
              return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected ';' after statement"})  
            }
        };
        Ok(Statement::PrintStatement(PrintStatement {token, value}))
    }

    // blockStatement -> block ;
    fn block_statement(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<BlockStatement, LoxError> {
        let statements = self.block(tokens)?;
        Ok(BlockStatement {statements})
    }
    
    // block -> "{" declaration* "}" ;
    fn block(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Vec<Statement>, LoxError> {
        tokens.next(); // consume "{"
        let mut statements = Vec::new();
        loop {
            match tokens.peek().unwrap().token_type {
                TokenType::RightBrace => {
                    tokens.next(); // consume "}"
                    break;
                },
                TokenType::EOF => {
                    return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "reached EOF while parsing, expected '}'"})
                }
                _ => {
                    statements.push(self.declaration(tokens)?);
                }
            }
        };
        Ok(statements)
    }

    // ifStatement -> "if" "(" expression ")" statement ("else" statement)? ;
    fn if_statement(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Statement, LoxError> {
        tokens.next(); // consume "if"
        
        match tokens.peek().unwrap().token_type {
            TokenType::LeftParen => {
                tokens.next(); // consume "("
            },
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected '(' after if"})
            }
        };

        let condition = self.expression(tokens)?;
        
        match tokens.peek().unwrap().token_type {
            TokenType::RightParen => {
                tokens.next(); // consume ")"
            },
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected ')' after if condition"})
            }
        };

        let then_branch = Box::new(self.statement(tokens)?);

        let else_branch = match tokens.peek().unwrap().token_type {
            TokenType::Else => {
                tokens.next(); // consume "else"
                Some(Box::new(self.statement(tokens)?))
            },
            _ => {
                None
            }
        };

        Ok(Statement::IfStatement(IfStatement {condition, then_branch, else_branch}))
    }

    // whileStatement -> "while" "(" expression ")" statement ;
    fn while_statement(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Statement, LoxError> {
        tokens.next(); // consume "while"
        
        match tokens.peek().unwrap().token_type {
            TokenType::LeftParen => {
                tokens.next(); // consume "("
            },
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected '(' after while"})
            }
        };

        let condition = self.expression(tokens)?;
        
        match tokens.peek().unwrap().token_type {
            TokenType::RightParen => {
                tokens.next(); // consume ")"
            },
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected ')' after while condition"})
            }
        };

        let body = Box::new(self.statement(tokens)?);
        Ok(Statement::WhileStatement(WhileStatement {condition, body}))
    }

    // forStatement -> "for" "(" (varDecl | exprStatement | ";") expression? ";" expression? ")" statement ; 
    fn for_statement(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Statement, LoxError> {
        tokens.next(); // consume "for"

        match tokens.peek().unwrap().token_type {
            TokenType::LeftParen => {
                tokens.next(); // consume "("
            },
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected '(' after for"})
            }
        };
        
        let initializer;

        match tokens.peek().unwrap().token_type {
            TokenType::Semicolon => {
                initializer = None;
            }
            TokenType::Var => {
                initializer = Some(self.var_declaration(tokens)?);
            },
            _ => {
                initializer = Some(self.expression_statement(tokens)?);
            }
        };

        let condition;

        match tokens.peek().unwrap().token_type {
            TokenType::Semicolon => {
                condition = Expr::Literal(Literal {
                    value: LiteralValue::BooleanValue(true), 
                    token: tokens.peek().unwrap().to_owned().to_owned() // yeah it gets the ";" token idk
                });
            }
            _ => {
                condition = self.expression(tokens)?;
            }
        };

        match tokens.peek().unwrap().token_type {
            TokenType::Semicolon => {
                tokens.next(); // consume ";"
            },
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected ';' after for condition"})
            }
        };

        let increment;

        match tokens.peek().unwrap().token_type {
            TokenType::Semicolon => {
                increment = None;
            }
            _ => {
                increment = Some(self.expression(tokens)?);
            }
        };

        match tokens.peek().unwrap().token_type {
            TokenType::RightParen => {
                tokens.next(); // consume ")"
            },
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected ')' after for clause"})
            }
        };

        let body = self.statement(tokens)?;

        // finished parsing, time to desugar

        let while_node = match increment {
            Some(increment) => {
                // if increment exists,
                // then create:
                /* 
                    while (condition) {
                        <body>
                        <increment>
                    }
                */
                let increment_statement = Statement::ExpressionStatement(ExpressionStatement {expression: increment});
                let block = Statement::BlockStatement(BlockStatement {statements: vec![body, increment_statement]});
                 Statement::WhileStatement(WhileStatement {condition, body: Box::new(block) }) 
            },
            None => {
                // if increment is none,
                // then create:
                /* 
                    while (condition)
                        <body>
                */
                Statement::WhileStatement(WhileStatement {condition, body: Box::new(body)})
            }
        };

        match initializer {
            // if initializer exists
            // then create
            /*
                {
                    <initializer>
                    <while_node>
                }
            */
            Some(initializer) => {
                Ok(Statement::BlockStatement(BlockStatement {statements: vec![initializer, while_node]}))
            }
            None => {
            // if initializer doesn't exist
            // just returned the previously created node
                Ok(while_node)
            }
        }
    }

    // returnStatement -> "return" expression? ";" ;
    fn return_statement(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Statement, LoxError> {
        let keyword = tokens.next().unwrap().to_owned(); // consume "return"
        let value;
        match &tokens.peek().unwrap().token_type {
            TokenType::Semicolon => {
                tokens.next(); // consume ";"
                value = None;
            },
            _ => {
                value = Some(self.expression(tokens)?);
                match &tokens.peek().unwrap().token_type {
                    TokenType::Semicolon => {
                        tokens.next(); // consume ";"
                    },
                    _ => {
                        return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected ';' after return statement"})  
                    }
                }
            }
        }
        Ok(Statement::ReturnStatement(ReturnStatement {keyword, value}))
    }

    // exprStatement -> expression ";" ;
    fn expression_statement(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Statement, LoxError> {
        let expr = self.expression(tokens)?;
        match &tokens.peek().unwrap().token_type {
            TokenType::Semicolon => {
                tokens.next(); // consume ";"
            },
            _ => {
              return Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "expected ';' after statement"})  
            }
        };
        Ok(Statement::ExpressionStatement(ExpressionStatement {expression: expr}))
    }

    // expression -> assignment ;
    fn expression(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, LoxError> {
        self.assignment(tokens)
    }

    // assignment -> (call ".")? IDENTIFIER "=" assignment | logic_or ;
    fn assignment(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, LoxError> {
        let expr = self.or(tokens)?;

        match &tokens.peek().unwrap().token_type {
            TokenType::Equal => {
                tokens.next().unwrap(); // consume "="
                let value = self.assignment(tokens)?;
                match expr {
                    Expr::Variable(v) => {
                        return Ok(Expr::Assignment(Assignment {token: v.token, value: Box::new(value)}));
                    },
                    Expr::Get(g) => {
                        let name = g.name;
                        let object = g.object;
                        return Ok(Expr::Set(Set {object, name, value: Box::new(value)}));
                    }
                    _ => {}
                };
                Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "invalid assignment target"})
            },
            _ => {
                Ok(expr)
            }
        }
    }

    // logic_or -> logic_and ( "or" logic_and )* ;
    fn or(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, LoxError> {
        let mut expr = self.and(tokens)?;
        loop {
            let operator;
            let token;
            match &tokens.peek().unwrap().token_type {
                TokenType::Or => {
                    token = tokens.next().unwrap();
                    operator = LogicalOperator::Or;
                },
                _ => break
            }
            let right = self.and(tokens)?;
            expr = Expr::Logical(Logical {token: token.to_owned(), operator: operator, left: Box::new(expr), right: Box::new(right)});
        };
        Ok(expr)
    }

    // logic_and -> equality ( "and" equality )* ;
    fn and(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, LoxError> {
        let mut expr = self.equality(tokens)?;
        loop {
            let operator;
            let token;
            match &tokens.peek().unwrap().token_type {
                TokenType::And => {
                    token = tokens.next().unwrap();
                    operator = LogicalOperator::And;
                },
                _ => break
            }
            let right = self.equality(tokens)?;
            expr = Expr::Logical(Logical {token: token.to_owned(), operator: operator, left: Box::new(expr), right: Box::new(right)});
        };
        Ok(expr)
    }

    // equality -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, LoxError> {
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
            expr = Expr::Binary(Binary {token: token.to_owned(), operator: operator, left: Box::new(expr), right: Box::new(right)});
        };
        Ok(expr)
    }

    // comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, LoxError> {
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
            expr = Expr::Binary(Binary {token: token.to_owned(), operator: operator, left: Box::new(expr), right: Box::new(right)});
        };
        Ok(expr)
    }

    // term -> factor ( ( "-" | "+") factor )* ;
    fn term(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, LoxError> {
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
            expr = Expr::Binary(Binary {token: token.to_owned(), operator: operator, left: Box::new(expr), right: Box::new(right)});
        }
        Ok(expr)
    }

    // factor -> unary ( ( "/" | "*") unary )* ;
    fn factor(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, LoxError>{
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
            expr = Expr::Binary(Binary {token: token.to_owned(), operator: operator, left: Box::new(expr), right: Box::new(right)});
        }
        Ok(expr)
    }

    // unary -> ( "!" | "-" ) unary | call ;
    fn unary(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, LoxError> {
        match &tokens.peek().unwrap().token_type {
            TokenType::Bang => {
                let token = tokens.next().unwrap();
                let operator = UnaryOperator::Bang;
                let right = self.unary(tokens)?;
                Ok(Expr::Unary(Unary {operator: operator, token: token.to_owned(), right: Box::new(right)}))
            },
            TokenType::Minus => {
                let token = tokens.next().unwrap();
                let operator = UnaryOperator::Minus;
                let right = self.unary(tokens)?;
                Ok(Expr::Unary(Unary {operator: operator, token: token.to_owned(), right: Box::new(right)}))
            }
            _ => {
                self.call(tokens)
            }
        }
    }
    
    // call -> primary ( "(" arguments? ")" |  "." IDENTIFIER )* ;
    fn call(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, LoxError> {
        let mut expr = self.primary(tokens)?;
        loop {
            match &tokens.peek().unwrap().token_type {
                TokenType::LeftParen => {
                    tokens.next(); // consume "("
                    // finish call
                    let mut args = Vec::new();
                    let token;
                    match &tokens.peek().unwrap().token_type {
                        TokenType::RightParen => {
                            // call has no arguments
                            token = tokens.next().unwrap().to_owned(); // consume ")"
                        },
                        _ => {
                            // call has arguments
                            args = self.arguments(tokens)?;
                            match &tokens.peek().unwrap().token_type {
                                TokenType::RightParen => {
                                    token = tokens.next().unwrap().to_owned(); // consume ")"
                                },
                                _ => {
                                    return Err(LoxError {kind: LoxErrorKind::ScannerError, message: "expected ')' call"})
                                }
                            }
                        }
                    };
                    expr = Expr::Call(Call {callee: Box::new(expr), arguments: args, token});
                },
                TokenType::Dot => {
                    tokens.next(); // consume "."
                    match &tokens.peek().unwrap().token_type {
                        TokenType::Identifier => {
                            let name = tokens.next().unwrap().to_owned(); // consume identifier
                            expr = Expr::Get(Get { object: Box::new(expr), name });
                        },
                        _ => {
                            return Err(LoxError {kind: LoxErrorKind::ScannerError, message: "expected identifier after '.'"})
                        }
                    }
                },
                _ => {
                    break;
                }
            };
        };
        Ok(expr)
    }
    
    // arguments -> expression ( "," expression )* ;
    fn arguments(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Vec<Expr>, LoxError> {
        let mut args: Vec<Expr> = Vec::new();
        loop {
            if args.len() > MAX_PARAMETERS {
                // no need to return the Error
                // that would mean the parser is in a bad state and needs to synchronize
                // but we don't need to do that for this type of error
                self.errors.push(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "can't have > 255 arguments to a function call"})
            }
            args.push(self.expression(tokens)?);
            match &tokens.peek().unwrap().token_type {
                TokenType::Comma => {
                    tokens.next(); // consume ","
                },
                _ => {
                    break;
                }
            }
        };
        Ok(args)
    }

    // primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | "this" | "super" "." IDENTIFIER ;
    fn primary(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, LoxError> {
        match &tokens.peek().unwrap().token_type {
            TokenType::False | TokenType::True | TokenType::Number | TokenType::String | TokenType::Nil => {
                let token = tokens.next().unwrap().to_owned();
                let value = token.literal.clone().unwrap();
                Ok(Expr::Literal(Literal { token, value }))
            },
            TokenType::This => {
                let keyword = tokens.next().unwrap().to_owned();
                Ok(Expr::This(This { keyword }))
            },
            TokenType::Super => {
                let keyword = tokens.next().unwrap().to_owned();
                match &tokens.peek().unwrap().token_type {
                    TokenType::Dot => {
                        tokens.next(); // consume '.'
                        match &tokens.peek().unwrap().token_type {
                            TokenType::Identifier => {
                                let method = tokens.next().unwrap().to_owned();
                                Ok(Expr::Super(Super { keyword, method }))
                            },
                            _ => {
                                Err(LoxError {kind: LoxErrorKind::ScannerError, message: "expected superclass method name after '.'"})
                            }
                        }
                    }, 
                    _ => {
                        Err(LoxError {kind: LoxErrorKind::ScannerError, message: "expected '.' after 'super' keyword"})
                    }
                }
            },
            TokenType::Identifier => {
                Ok(Expr::Variable(Variable { token: tokens.next().unwrap().to_owned() }))
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
                Err(LoxError {kind: LoxErrorKind::SyntaxError(tokens.peek().unwrap().line), message: "invalid syntax"})
            }
        }
    }
}