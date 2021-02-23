use std::{cell::RefCell, collections::HashMap, fmt::{Display}, rc::Rc};

use crate::{ast::{Assignent, Binary, BinaryOperator, BlockStatement, Call, Expr, ExpressionStatement, FunDeclStatement, IfStatement, Literal, Logical, LogicalOperator, PrintStatement, ReturnStatement, Statement, Unary, UnaryOperator, VarDeclStatement, Variable, WhileStatement}, error::{LoxError, LoxErrorKind}, output::{Outputter, Printer}, tokens::LiteralValue};
use crate::callable::Function;

#[derive(Debug)]
pub(crate) struct TreeWalker<T: Outputter> {
    pub environment: Rc<RefCell<Environment>>,
    pub outputter: T,
}

#[derive(Debug, Clone)]
pub(crate) struct Environment {
    pub values: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment { values: HashMap::new(), parent: None }
    }

    pub fn define<'b>(&mut self, name: &'b str, value: Value) {
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
            None => {
                match &self.parent {
                    Some(parent) => {
                        parent.borrow().get(name)
                    }
                    None => {
                        Err(LoxError {kind: LoxErrorKind::NameError, message: "name is not defined"})
                    }
                }   
            }
        }
    }

    fn assign<'b>(&mut self, name: &'b str, value: &Value) -> Result<(), LoxError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value.clone());
            Ok(())
        } else {
            match &mut self.parent {
                Some(parent) => {
                    parent.borrow_mut().assign(name, value)
                },
                None => {
                    Err(LoxError {kind: LoxErrorKind::NameError, message: "name is not defined"})
                }
            }
        } 
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Keys: {:?}, Has Parent?: {}", self.values.keys(), match self.parent { Some(_) => "yes", None => "no"})
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Value {
    NumberValue(f64),
    StringValue(String),
    BooleanValue(bool),
    NilValue,
    Callable(Function),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::NumberValue(n) => write!(f, "{}", n),
            Value::StringValue(n) => write!(f, "{}", n),
            Value::BooleanValue(n) => write!(f, "{}", n),
            Value::NilValue => write!(f, "nil"),
            Value::Callable(_) => write!(f, "callable"),
        }
    }
}

impl<T: Outputter> TreeWalker<T> {
    pub fn new() -> TreeWalker<Printer> {
        TreeWalker { environment: Rc::new(RefCell::new(Environment::new())), outputter: Printer{} }
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
            Statement::FunDeclStatement(f) => {
                self.visit_fun_decl_statement(f)
            }
            Statement::ReturnStatement(r) => {
                self.visit_return_statement(r)
            }
        }
    }

    fn visit_print_statement(&mut self, stmt: &PrintStatement) -> Result<(), LoxError> {
        let value = self.visit_expr(&stmt.value)?;
        self.outputter.output_value(value);
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
        self.define(&stmt.token.lexeme, initial_value);
        Ok(())
    }

    fn visit_block_statement<'b>(&mut self, stmt: &'b BlockStatement) -> Result<(), LoxError> {
        let mut env = Environment::new();
        env.parent = Some(Rc::clone(&self.environment));
        self.execute_block(&stmt.statements, Rc::new(RefCell::new(env)))
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

    fn visit_fun_decl_statement<'b>(&mut self, stmt: &'b FunDeclStatement) -> Result<(), LoxError> {
        let fun = Function::new(stmt.to_owned(), Rc::clone(&self.environment));
        self.define(&stmt.name.lexeme, Value::Callable(fun));
        Ok(())
    }

    fn visit_return_statement<'b>(&mut self, stmt: &'b ReturnStatement) -> Result<(), LoxError> {
        match &stmt.value {
            Some(expr) => {
                let value = self.visit_expr(&expr)?;
                Err(LoxError {kind: LoxErrorKind::Return(value), message: ""})
            },
            _ => {
                Ok(())
            }
        }
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
                self.visit_call(c)
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
        self.get(&expr.token.lexeme)
    }

    fn visit_assignment(&mut self, expr: &Assignent) -> Result<Value, LoxError> {
        let value = self.visit_expr(expr.value.as_ref())?;
        self.assign(&expr.token.lexeme, value)
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

    fn visit_call(&mut self, expr: &Call) -> Result<Value, LoxError> {
        let callee = self.visit_expr(expr.callee.as_ref())?;
        let mut args = Vec::new();
        // argument expressions evaluated from left to right
        for arg in &expr.arguments {
            args.push(self.visit_expr(&arg)?)
        }
        match callee {
            Value::Callable(callee) => {
                if args.len() != callee.arity() {
                    Err(LoxError {kind: LoxErrorKind::TypeError, message: "Got wrong number of arguments"})
                } else {
                    callee.call(self, args)
                }
            },
            _ => {
                Err(LoxError {kind: LoxErrorKind::TypeError, message: "expression is not callable"})
            }
        }
    }

    pub fn execute_block(&mut self, statements: &Vec<Statement>, env: Rc<RefCell<Environment>>) -> Result<(), LoxError> {
        let previous_env = Rc::clone(&self.environment);
        self.environment = env;
        for statement in statements {
            let result = self.visit_statement(statement);
            if let Err(e) = result {
                // clean up on error
                self.environment = previous_env;
                return Err(e)
            }
        }
        // clean up
        self.environment = previous_env;
        Ok(())
    }

    fn define<'b>(&mut self, name: &'b str, value: Value) {
        self.environment.borrow_mut().define(name, value)
    }

    fn get<'b>(&self, name: &'b str) -> Result<Value, LoxError> {
        self.environment.borrow().get(name)
    }

    fn assign<'b>(&mut self, name: &'b str, value: Value) -> Result<Value, LoxError> {
        match self.environment.borrow_mut().assign(name, &value) {
            Ok(_) => Ok(value),
            Err(e) => Err(e)
        }
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
            Value::Callable(_) => true,
        }
    }
}