
use core::fmt;
use std::error::Error;

use crate::tree_walker::Value;

#[derive(Debug)]
pub(crate) struct LoxError {
    pub message: &'static str,
    pub kind: LoxErrorKind
}

#[derive(Debug)]
pub(crate) enum LoxErrorKind {
    ScannerError,
    SyntaxError(i32),
    TypeError,
    NameError,
    RuntimeError,
    ResolvingError,
    AttributeError,
    Return(Value), // dirty hack
}

impl Error for LoxError {}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            LoxErrorKind::ScannerError => write!(f, "ScannerError"),
            LoxErrorKind::SyntaxError(line) => write!(f, "SyntaxError: line {}", line),
            LoxErrorKind::TypeError => write!(f, "TypeError"),
            LoxErrorKind::NameError => write!(f, "NameError"),
            LoxErrorKind::Return(_) => write!(f, "ReturnValue"),
            LoxErrorKind::RuntimeError => {write!(f, "RuntimeError")},
            LoxErrorKind::ResolvingError => {write!(f, "ResolvingError")},
            LoxErrorKind::AttributeError => {write!(f, "AttributeError")},
        }
    }
}
