use std::fmt;

/// A table which implements the `Display` trait
pub struct TableDisplay {
    inner: String,
}

impl TableDisplay {
    pub(crate) fn new(inner: Vec<u8>) -> Self {
        TableDisplay {
            inner: String::from_utf8(inner).expect("valid utf8 string"),
        }
    }
}

impl fmt::Display for TableDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner.trim())
    }
}
