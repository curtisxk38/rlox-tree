use std::{fmt::Display, time::SystemTime};

use crate::{callable::LoxCallable, error::LoxError, error::LoxErrorKind::RuntimeError, tree_walker::Value};


#[derive(Debug, Clone)]
pub(crate) struct ClockCallable {}

impl LoxCallable for ClockCallable {
    fn call(& self, _interpreter:  &mut crate::tree_walker::TreeWalker, _arguments: Vec<crate::tree_walker::Value>) -> Result<crate::tree_walker::Value, crate::error::LoxError> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => { Ok(Value::NumberValue(n.as_secs() as f64)) }
            Err(_) => { Err(LoxError {kind: RuntimeError, message: "System time before unix epoch" })}
        }
    }

    fn arity(&self) -> usize {
        0
    }
}

impl Display for ClockCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn clock>")
    }
}