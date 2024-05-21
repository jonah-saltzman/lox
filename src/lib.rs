pub mod cli;
pub mod scan;
pub mod object;
mod token;

use ascii::AsciiStr;
use thiserror::Error;
use object::Object;
use scan::{Scanner, ScanError};

#[derive(Error, Debug)]
pub enum LoxError {
    #[error("scan error")]
    Scan(#[from] ScanError),
}

pub struct JLox {
    had_error: bool
}

impl JLox {

    pub fn new() -> Self {
        Self {
            had_error: false
        }
    }

    pub fn handle_source(&mut self, source: &AsciiStr) -> Result<Object, LoxError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan()?;
        let mut last: Object = Object::None;
        for t in tokens {
            println!("{:?}", t);
            last = t.into_object()
        }
        Ok(last)
    }

    pub fn error(&self, line: i64, span: &str, message: &str) {
        eprintln!("[line {line}] Error{span}: {message}")
    }

    pub fn reset_error(&mut self) {
        self.had_error = false
    }
}