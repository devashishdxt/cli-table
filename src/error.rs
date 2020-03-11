use std::fmt;

/// Errors returned by functions in this crate
#[derive(Debug)]
pub enum Error {
    /// Returned when there is a mismatch in number of columns in different rows of a [`Table`](struct.Table.html)
    MismatchedColumns,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MismatchedColumns => write!(
                f,
                "All rows in the table should have same number of cells/columns"
            ),
        }
    }
}
