
use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct LoxError {
    pub message: &'static str,
    pub kind: LoxErrorKind
}

#[derive(Debug)]
pub enum LoxErrorKind {
    ScannerError
}

impl Error for LoxError {}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            LoxErrorKind::ScannerError => write!(f, "ScannerError")
        }
    }
}
