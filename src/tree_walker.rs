use std::{collections::HashMap, fmt::{Display}};

use crate::{ast::{Assignent, Binary, BinaryOperator, Expr, ExpressionStatement, Literal, PrintStatement, Statement, Unary, UnaryOperator, VarDeclStatement, Variable}, error::{LoxError, LoxErrorKind}, tokens::LiteralValue};


#[derive(Debug)]
pub(crate) struct TreeWalker {
    environment: Environment
}

#[derive(Debug)]
struct Environment {
    values: HashMap<String, Value>
}

impl Environment {
    fn new() -> Environment {
        Environment { values: HashMap::new() }
    }

    fn define<'b>(&mut self, name: &'b str, value: Value) {
        self.values.insert(name.to_string(), value);
        // this means you can redine values
        // valid program:
        /*
        var a = "first";
        print a; //"first"
        var a = "second";
        print a; //"second"
        */
    }

    fn get<'b>(&self, name: &'b str) -> Result<Value, LoxError> {
        let result = self.values.get(name);
        match result {
            Some(v) => Ok(v.clone()),
            None => Err(LoxError {kind: LoxErrorKind::NameError, message: "name is not defined"})
        }
    }

    fn assign<'b>(&mut self, name: &'b str, value: Value) -> Result<Value, LoxError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value.clone());
            Ok(value)
        } else {
            Err(LoxError {kind: LoxErrorKind::NameError, message: "name is not defined" })
        }
        
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Value {
    NumberValue(f64),
    StringValue(String),
    BooleanValue(bool),
    NilValue
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::NumberValue(n) => write!(f, "{}", n),
            Value::StringValue(n) => write!(f, "{}", n),
            Value::BooleanValue(n) => write!(f, "{}", n),
            Value::NilValue => write!(f, "nil"),
        }
    }
}

impl TreeWalker {
    pub fn new() -> TreeWalker {
        TreeWalker { environment: Environment::new() }
    }
    
    pub fn visit_statement<'b>(&mut self, stmt: &'b Statement) -> Result<(), LoxError> {
        match stmt {
            Statement::PrintStatement(p) => {
                self.visit_print_statement(p)
            },
            Statement::ExpressionStatement(e) => {
                self.visit_expression_statement(e)
            }
            Statement::VarDeclStatement(v) => {
                self.visit_var_decl_statement(v)
            }
        }
    }

    fn visit_print_statement(&mut self, stmt: &PrintStatement) -> Result<(), LoxError> {
        let value = self.visit_expr(&stmt.value)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_expression_statement(&mut self, stmt: &ExpressionStatement) -> Result<(), LoxError> {
        self.visit_expr(&stmt.expression)?;
        Ok(())
    }

    fn visit_var_decl_statement<'b>(&mut self, stmt: &'b VarDeclStatement) -> Result<(), LoxError> {
        // if you declare a variable without initializing it, it gets set to nil
        // var x; // x is nil
        let initial_value = match &stmt.initializer {
            Some(e) => self.visit_expr(e)?,
            None => Value::NilValue
        };
        self.environment.define(stmt.token.lexeme, initial_value);
        Ok(())
    }

    fn visit_expr(&mut self, expr: &Expr) -> Result<Value, LoxError> {
        match expr {
            Expr::Binary(e) => {
                self.visit_binary(e)
            }
            Expr::Unary(e) => {
                self.visit_unary(e)
            }
            Expr::Literal(e) => {
                self.visit_literal(e)
            }
            Expr::Variable(e) => {
                self.visit_variable(e)
            }
            Expr::Assignent(e) => {
                self.visit_assignment(e)
            }
        }
    }

    fn visit_binary(&mut self, expr: &Binary) -> Result<Value, LoxError> {
        let left = self.visit_expr(expr.left.as_ref())?;
        let right = self.visit_expr(expr.right.as_ref())?;
        match expr.operator {
            BinaryOperator::BangEqual => {
                Ok(Value::BooleanValue(!self.is_equal(&left, &right)))
            }
            BinaryOperator::EqualEqual => {
                Ok(Value::BooleanValue(self.is_equal(&left, &right)))
            }
            BinaryOperator::Greater => {
                match (left, right) {
                    (Value::NumberValue(l), Value::NumberValue(r)) => {
                        Ok(Value::BooleanValue(l > r))
                    }
                    _ => Err(LoxError {kind: LoxErrorKind::TypeError, message: "unsupported operand types"})
                }
            }
            BinaryOperator::GreaterEqual => {
                match (left, right) {
                    (Value::NumberValue(l), Value::NumberValue(r)) => {
                        Ok(Value::BooleanValue(l >= r))
                    }
                    _ => Err(LoxError {kind: LoxErrorKind::TypeError, message: "unsupported operand types"})
                }
            }
            BinaryOperator::Less => {
                match (left, right) {
                    (Value::NumberValue(l), Value::NumberValue(r)) => {
                        Ok(Value::BooleanValue(l < r))
                    }
                    _ => Err(LoxError {kind: LoxErrorKind::TypeError, message: "unsupported operand types"})
                }
            }
            BinaryOperator::LessEqual => {
                match (left, right) {
                    (Value::NumberValue(l), Value::NumberValue(r)) => {
                        Ok(Value::BooleanValue(l <= r))
                    }
                    _ => Err(LoxError {kind: LoxErrorKind::TypeError, message: "unsupported operand types"})
                }
            }
            BinaryOperator::Minus => {
                match (left, right) {
                    (Value::NumberValue(l), Value::NumberValue(r)) => {
                        Ok(Value::NumberValue(l - r))
                    }
                    _ => Err(LoxError {kind: LoxErrorKind::TypeError, message: "unsupported operand types"})
                }
            }
            BinaryOperator::Plus => {
                match (left, right) {
                    (Value::NumberValue(l), Value::NumberValue(r)) => {
                        Ok(Value::NumberValue(l + r))
                    },
                    (Value::StringValue(l), Value::StringValue(r)) => {
                        Ok(Value::StringValue(format!("{}{}", l, r)))
                    }
                    _ => Err(LoxError {kind: LoxErrorKind::TypeError, message: "unsupported operand types"})
                }
            },
            BinaryOperator::Slash => {
                match (left, right) {
                    (Value::NumberValue(l), Value::NumberValue(r)) => {
                        Ok(Value::NumberValue(l / r))
                    }
                    _ => Err(LoxError {kind: LoxErrorKind::TypeError, message: "unsupported operand types"})
                }
            }
            BinaryOperator::Star => {
                match (left, right) {
                    (Value::NumberValue(l), Value::NumberValue(r)) => {
                        Ok(Value::NumberValue(l * r))
                    }
                    _ => Err(LoxError {kind: LoxErrorKind::TypeError, message: "unsupported operand types"})
                }
            },
        }
    }

    fn visit_unary(&mut self, expr: &Unary) -> Result<Value, LoxError> {
        let right = self.visit_expr(expr.right.as_ref())?;
        match &expr.operator {
            UnaryOperator::Bang => {
                Ok(Value::BooleanValue(self.is_truthy(&right)))
            },
            UnaryOperator::Minus => {
                match right {
                    Value::NumberValue(n) => Ok(Value::NumberValue(n * -1.0)),
                    _ => Err(LoxError {kind: LoxErrorKind::TypeError, message: "unsupported operant types"})
                }
            }
        }
    }

    fn visit_literal(&self, expr: &Literal) -> Result<Value, LoxError> {
        match &expr.value {
            LiteralValue::NumberValue(n) => Ok(Value::NumberValue(n.to_owned())),
            LiteralValue::StringValue(s) => Ok(Value::StringValue(s.to_owned())),
            LiteralValue::BooleanValue(b) => Ok(Value::BooleanValue(b.to_owned())),
            LiteralValue::NilValue => Ok(Value::NilValue)
        }
    }

    fn visit_variable(&self, expr: &Variable) -> Result<Value, LoxError> {
        self.environment.get(expr.token.lexeme)
    }

    fn visit_assignment(&mut self, expr: &Assignent) -> Result<Value, LoxError> {
        let value = self.visit_expr(expr.value.as_ref())?;
        self.environment.assign(expr.token.lexeme, value)
    }

    fn is_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::NumberValue(l), Value::NumberValue(r)) => {
               l == r
            },
            (Value::StringValue(l), Value::StringValue(r)) => {
               l == r
            },
            (Value::BooleanValue(l), Value::BooleanValue(r)) => {
               l == r
            },
            (Value::NilValue, Value::NilValue) => {
               true
            }
            _ => false
        }
    }

    fn is_truthy(&self, val: &Value) -> bool {
        // false and nil are falsey, everything else is truthy
        match val {
            Value::BooleanValue(b) => b.to_owned(),
            Value::NilValue => false,
            Value::NumberValue(_) => true,
            Value::StringValue(_) => true,
        }
    }
}