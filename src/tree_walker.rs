use std::{collections::HashMap, fmt::{Display}, vec};

use crate::{ast::{Assignent, Binary, BinaryOperator, BlockStatement, Expr, ExpressionStatement, IfStatement, Literal, Logical, LogicalOperator, PrintStatement, Statement, Unary, UnaryOperator, VarDeclStatement, Variable, WhileStatement}, error::{LoxError, LoxErrorKind}, tokens::LiteralValue};


#[derive(Debug)]
pub(crate) struct TreeWalker {
    environments: Vec<Environment>,
}

#[derive(Debug)]
struct Environment {
    values: HashMap<String, Value>,
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

    fn assign<'b>(&mut self, name: &'b str, value: &Value) -> Result<(), LoxError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value.clone());
            Ok(())
        } else {
            Err(LoxError {kind: LoxErrorKind::NameError, message: "name is not defined"})
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
        let environments =vec![Environment::new()];
        TreeWalker { environments: environments }
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
            Statement::BlockStatement(s) => {
                self.visit_block_statement(s)
            }
            Statement::IfStatement(i) => {
                self.visit_if_statement(i)
            }
            Statement::WhileStatement(w) => {
                self.visit_while_statement(w)
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
        self.define(stmt.token.lexeme, initial_value);
        Ok(())
    }

    fn visit_block_statement<'b>(&mut self, stmt: &'b BlockStatement) -> Result<(), LoxError> {
        let env = Environment::new();
        self.execute_block(&stmt.statements, env)
    }

    fn visit_if_statement<'b>(&mut self, stmt: &'b IfStatement) -> Result<(), LoxError> {
        let condition = self.visit_expr(&stmt.condition)?;
        if self.is_truthy(&condition) {
            self.visit_statement(stmt.then_branch.as_ref())
        } else if let Some(else_branch) = &stmt.else_branch{
            self.visit_statement(else_branch.as_ref())
        } else {
            Ok(())
        }
    }

    fn visit_while_statement<'b>(&mut self, stmt: &'b WhileStatement) -> Result<(), LoxError> {
        loop {
            let condition = self.visit_expr(&stmt.condition)?;
            if !self.is_truthy(&condition) {
                break;
            }
            self.visit_statement(stmt.body.as_ref())?;
        }
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
            },
            Expr::Grouping(e) => {
                self.visit_expr(e.expr.as_ref())
            },
            Expr::Variable(e) => {
                self.visit_variable(e)
            }
            Expr::Assignent(e) => {
                self.visit_assignment(e)
            }
            Expr::Logical(l) => {
                self.visit_logical(l)
            }
            Expr::Call(c) => {
                todo!()
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
        self.get(expr.token.lexeme)
    }

    fn visit_assignment(&mut self, expr: &Assignent) -> Result<Value, LoxError> {
        let value = self.visit_expr(expr.value.as_ref())?;
        self.assign(expr.token.lexeme, value)
    }

    fn visit_logical(&mut self, expr: &Logical) -> Result<Value, LoxError> {
        let left_value = self.visit_expr(expr.left.as_ref())?;
        match expr.operator {
            LogicalOperator::And => {
                if !self.is_truthy(&left_value) {
                    return Ok(left_value);
                }
            }
            LogicalOperator::Or => {
                if self.is_truthy(&left_value) {
                    return Ok(left_value);
                }
            }
        };
        let right_value = self.visit_expr(expr.right.as_ref())?;
        return Ok(Value::BooleanValue(self.is_truthy(&right_value)));
    }

    fn execute_block(&mut self, statements: &Vec<Statement>, env: Environment) -> Result<(), LoxError> {
        self.environments.push(env);
        for statement in statements {
            let result = self.visit_statement(statement);
            if let Err(e) = result {
                // clean up on error
                self.environments.pop();
                return Err(e)
            }
        }
        self.environments.pop();
        Ok(())
    }

    fn define<'b>(&mut self, name: &'b str, value: Value) {
        // by unwrapping, we assume that environments always has at least 1 env
        self.environments.last_mut().unwrap().define(name, value)
    }

    fn get<'b>(&self, name: &'b str) -> Result<Value, LoxError> {
        for env in self.environments.iter().rev() {
            match env.get(name) {
                Ok(v) => return Ok(v),
                Err(_) => {}
            }
        }
        Err(LoxError {kind: LoxErrorKind::NameError, message: "name is not defined"})
    }

    fn assign<'b>(&mut self, name: &'b str, value: Value) -> Result<Value, LoxError> {
        for env in self.environments.iter_mut().rev() {
            match env.assign(name, &value) {
                Ok(_) => return Ok(value),
                Err(_) => {}
            }
        }

        Err(LoxError {kind: LoxErrorKind::NameError, message: "name is not defined"})
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