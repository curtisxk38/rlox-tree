use std::{iter::Peekable, slice::Iter};

use crate::{ast::{Assignent, Binary, BlockStatement, Call, Expr, ExpressionStatement, Grouping, IfStatement, Literal, Logical, LogicalOperator, PrintStatement, Statement, Unary, UnaryOperator, VarDeclStatement, Variable, WhileStatement}, error::{LoxError, LoxErrorKind}, tokens::{LiteralValue, Token, TokenType}};
use crate::ast::{BinaryOperator};


pub(crate) struct Parser {
    errors: Vec<LoxError>
}

impl Parser {

    pub fn new() -> Parser {
        Parser { errors: Vec::new() }
    }

    // program -> statement* EOF ;
    pub fn parse<'a>(&mut self, tokens: &'a Vec<Token>) -> Result<Vec<Statement<'a>>, Vec<LoxError>> {
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
            Err(self.errors)
        } else {
            Ok(statements)
        }
    }

    fn synchronize<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) {
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
    fn declaration<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
        match &tokens.peek().unwrap().token_type {
            TokenType::Var => {
                self.var_declaration(tokens)
            },
            _ => self.statement(tokens)
        }
    }

    fn var_declaration<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
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
    // | printStatement 
    // | blockStatement 
    // | ifStatement
    // | whileStatement 
    // | forStatement ;
    fn statement<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
        match &tokens.peek().unwrap().token_type {
            TokenType::Print => {
                self.print_statement(tokens)
            },
            TokenType::LeftBrace => {
                self.block_statement(tokens)
            },
            TokenType::If => {
                self.if_statement(tokens)
            },
            TokenType::While => {
                self.while_statement(tokens)
            },
            TokenType::For => {
                self.for_statement(tokens)
            }
            _ => {
                // if the next token doesn't like any other statement, assume its an expr statement
                self.expression_statement(tokens)
            }
        }
    }

    // printStatement -> "print" expression ";" ;
    fn print_statement<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
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

    // blockStatement -> "{" declaration* "}" ;
    fn block_statement<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
        tokens.next(); // consume "{"
        let mut statements = Vec::new();
        loop {
            match tokens.peek().unwrap().token_type {
                TokenType::RightBrace => {
                    tokens.next(); // consume "}"
                    break;
                },
                TokenType::EOF => {
                    return Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "reached EOF while parsing, expected '}'"})
                }
                _ => {
                    statements.push(self.declaration(tokens)?);
                }
            }
        };
        Ok(Statement::BlockStatement(BlockStatement {statements}))
    }

    // ifStatement -> "if" "(" expression ")" statement ("else" statement)? ;
    fn if_statement<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
        tokens.next(); // consume "if"
        
        match tokens.peek().unwrap().token_type {
            TokenType::LeftParen => {
                tokens.next(); // consume "("
            },
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "expected '(' after if"})
            }
        };

        let condition = self.expression(tokens)?;
        
        match tokens.peek().unwrap().token_type {
            TokenType::RightParen => {
                tokens.next(); // consume ")"
            },
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "expected ')' after if condition"})
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
    fn while_statement<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
        tokens.next(); // consume "while"
        
        match tokens.peek().unwrap().token_type {
            TokenType::LeftParen => {
                tokens.next(); // consume "("
            },
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "expected '(' after while"})
            }
        };

        let condition = self.expression(tokens)?;
        
        match tokens.peek().unwrap().token_type {
            TokenType::RightParen => {
                tokens.next(); // consume ")"
            },
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "expected ')' after while condition"})
            }
        };

        let body = Box::new(self.statement(tokens)?);
        Ok(Statement::WhileStatement(WhileStatement {condition, body}))
    }

    // forStatement -> "for" "(" (varDecl | exprStatement | ";") expression? ";" expression? ")" statement ; 
    fn for_statement<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
        tokens.next(); // consume "for"

        match tokens.peek().unwrap().token_type {
            TokenType::LeftParen => {
                tokens.next(); // consume "("
            },
            _ => {
                return Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "expected '(' after for"})
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
                    token: tokens.peek().unwrap() // yeah it gets the ";" token idk
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
                return Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "expected ';' after for condition"})
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
                return Err(LoxError {kind: LoxErrorKind::SyntaxError, message: "expected ')' after for clause"})
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

    // exprStatement -> expression ";" ;
    fn expression_statement<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Statement<'a>, LoxError> {
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
    fn expression<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
        self.assignment(tokens)
    }

    // assignment -> IDENTIFIER "=" assignment | logic_or ;
    fn assignment<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
        let expr = self.or(tokens)?;

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

    // logic_or -> logic_and ( "or" logic_and )* ;
    fn or<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
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
            expr = Expr::Logical(Logical {token: token, operator: operator, left: Box::new(expr), right: Box::new(right)});
        };
        Ok(expr)
    }

    // logic_and -> equality ( "and" equality )* ;
    fn and<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
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
            expr = Expr::Logical(Logical {token: token, operator: operator, left: Box::new(expr), right: Box::new(right)});
        };
        Ok(expr)
    }

    // equality -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
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
    fn term<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
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
    fn factor<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError>{
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

    // unary -> ( "!" | "-" ) unary | call ;
    fn unary<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
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
                self.call(tokens)
            }
        }
    }
    
    // call -> primary ( "(" arguments? ")" )* ;
    fn call<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
        let p = self.primary(tokens)?;
        let mut args = Vec::new();
        let token;
        match &tokens.peek().unwrap().token_type {
            TokenType::LeftParen => {
                tokens.next(); // consume "("
                match &tokens.peek().unwrap().token_type {
                    TokenType::RightParen => {
                        token = tokens.next().unwrap(); // consume ")"
                    },
                    _ => {
                        args = self.arguments(tokens)?;
                        match &tokens.peek().unwrap().token_type {
                            TokenType::RightParen => {
                                token = tokens.next().unwrap(); // consume ")"
                            },
                            _ => {
                                return Err(LoxError {kind: LoxErrorKind::ScannerError, message: "expected ')' call"})
                            }
                        }
                    }
                }
            },
            _ => {
                return Ok(p);
            }
        };
        Ok(Expr::Call(Call {callee: Box::new(p), arguments: args, token}))
    }

    // arguments -> expression ( "," expression )* ;
    fn arguments<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Vec<Expr<'a>>, LoxError> {
        let mut args: Vec<Expr> = Vec::new();
        loop {
            if args.len() > 255 {
                self.errors.push(LoxError {kind: LoxErrorKind::SyntaxError, message: "can't have > 255 arguments to a function call"})
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

    // primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary<'a>(&mut self, tokens: &mut Peekable<Iter<'a, Token>>) -> Result<Expr<'a>, LoxError> {
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