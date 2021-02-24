use std::{cell::RefCell, fmt::{Debug, Display}, rc::Rc};

use crate::{ast::FunDeclStatement, error::{LoxError, LoxErrorKind}, tree_walker::{Environment, TreeWalker, Value}};

pub(crate) trait LoxCallable: Display + Debug + Clone {
    fn call(& self, interpreter:  &mut TreeWalker, arguments: Vec<Value>) -> Result<Value, LoxError>;

    fn arity(&self) -> usize;
}

#[derive(Debug, Clone)]
pub(crate) struct Function {
    declaration: FunDeclStatement,
    closure: Rc<RefCell<Environment>>
}

impl Function {
    pub fn new(declaration: FunDeclStatement, closure: Rc<RefCell<Environment>>) -> Function {
        Function { declaration, closure }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme)
    }
}

impl LoxCallable for Function {

    fn call(& self, interpreter:  &mut TreeWalker, mut arguments: Vec<Value>) -> Result<Value, LoxError>{
        let mut env = Environment::new();
        env.parent = Some(Rc::clone(&self.closure));
        // ASSUMPTION made: arguments.len() = self.declaration.parameters.len()
        for parameter in &self.declaration.parameters {
            // get the first item in the list
            let arg = arguments.remove(0);
            env.define(&parameter.lexeme, arg)
        }

        let result = interpreter.execute_block(&self.declaration.body.statements, Rc::new(RefCell::new(env)));
        match result {
            Ok(_) => {
                Ok(Value::NilValue)
            },
            Err(e) => {
                match e.kind {
                    LoxErrorKind::Return(value) => {
                        Ok(value)
                    },
                    _ => {
                        Err(e)
                    }
                }
            }
        }
    }

    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }
}