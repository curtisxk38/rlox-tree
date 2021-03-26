use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{callable::LoxCallable, error::LoxError, tree_walker::{self, Value}};


#[derive(Debug, Clone)]
pub(crate) struct LoxClass {
    pub name: String,

}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

impl LoxCallable for LoxClass {
    fn call(& self, _interpreter:  &mut tree_walker::TreeWalker, _arguments: Vec<tree_walker::Value>) -> Result<tree_walker::Value, LoxError> {
        let instance = LoxInstance { class: self.clone() };
        Ok(Value::InstanceValue(Rc::new(RefCell::new(instance))))
    }

    fn arity(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LoxInstance {
    pub class: LoxClass,
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} instance>", self.class.name)
    }
}