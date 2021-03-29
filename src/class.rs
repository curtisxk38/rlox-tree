use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::{callable::{Function, LoxCallable}, error::{LoxError, LoxErrorKind}, tree_walker::{self, Value}};


#[derive(Debug, Clone)]
pub(crate) struct LoxClass {
    pub name: String,
    methods: HashMap<String, Function>,

}

impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, Function>) -> LoxClass {
        LoxClass { name, methods }
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

impl LoxCallable for LoxClass {
    fn call(& self, _interpreter:  &mut tree_walker::TreeWalker, _arguments: Vec<tree_walker::Value>) -> Result<tree_walker::Value, LoxError> {
        let instance = LoxInstance::new(self.clone());
        Ok(Value::InstanceValue(Rc::new(RefCell::new(instance))))
    }

    fn arity(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, Value>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> LoxInstance {
        LoxInstance { class, fields: HashMap::new() }
    }

    pub fn get(&self, name: &str) -> Result<Value, LoxError> {
        if let Some(value) = self.fields.get(name) {
            Ok(value.clone())
        } else if let Some(method) = self.class.methods.get(name) {
            Ok(Value::Callable(Box::new(method.clone())))
        } else {
            Err(LoxError {kind: LoxErrorKind::AttributeError, message: "Instance has no attribute with that name"})
        }
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.fields.insert(name.to_owned(), value);
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} instance>", self.class.name)
    }
}