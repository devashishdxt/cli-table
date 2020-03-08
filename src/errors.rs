use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct TableSizeError(&'static str);

impl fmt::Display for TableSizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for TableSizeError { }

impl TableSizeError {
    pub fn new(message: &'static str) -> TableSizeError {
        TableSizeError(message)
    }
}