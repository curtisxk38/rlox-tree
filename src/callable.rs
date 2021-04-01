use std::{cell::RefCell, fmt::{Debug, Display}, rc::Rc};

use crate::{ast::FunDeclStatement, class::LoxInstance, error::{LoxError, LoxErrorKind}, tree_walker::{Environment, TreeWalker, Value}};

pub(crate) trait LoxCallable: Display + Debug + LoxCallableClone {
    fn call(& self, interpreter:  &mut TreeWalker, arguments: Vec<Value>) -> Result<Value, LoxError>;

    fn arity(&self) -> usize;
}

pub(crate) trait LoxCallableClone {
    fn clone_box(&self) -> Box<dyn LoxCallable>;
}

impl<T> LoxCallableClone for T where
    T: 'static + LoxCallable + Clone {
        fn clone_box(&self) -> Box<dyn LoxCallable> {
            Box::new(self.clone())
        }
    }

impl Clone for Box<dyn LoxCallable> {
    fn clone(&self) -> Box<dyn LoxCallable> {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Function {
    declaration: FunDeclStatement,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool,
}

impl Function {
    pub fn new(declaration: FunDeclStatement, closure: Rc<RefCell<Environment>>, is_initializer: bool) -> Function {
        Function { declaration, closure, is_initializer }
    }

    pub fn bind(&self, instance: &Rc<RefCell<LoxInstance>>) -> Function {
        let mut environment = Environment::new();
        environment.parent = Some(Rc::clone(&self.closure));
        environment.define("this", Value::InstanceValue(Rc::clone(instance)));
        return Function::new(self.declaration.clone(), Rc::new(RefCell::new(environment)), self.is_initializer);
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

        let result = interpreter.execute_block(&self.declaration.body, Rc::new(RefCell::new(env)));
        match result {
            Ok(_) => {
                if self.is_initializer {
                    self.closure.borrow().get_at("this", 0)
                } else {
                    Ok(Value::NilValue)
                }
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