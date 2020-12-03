#![forbid(unsafe_code)]
#![cfg_attr(not(any(docsrs, feature = "doc")), forbid(unstable_features))]
#![deny(missing_docs)]
#![cfg_attr(any(docsrs, feature = "doc"), feature(doc_cfg))]
//! Rust crate for printing tables on command line.
//!
//! ## Usage
//!
//! Add `cli-table` in your `Cargo.toms`'s `dependencies` section
//!
//! ```toml
//! [dependencies]
//! cli-table = "0.4"
//! ```
//!
//! To create a table,
//!
//! ```rust
//! # use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
//! let table = vec![
//!     vec!["Name".cell().bold(true), "Age (in years)".cell().bold(true)],
//!     vec!["Tom".cell(), 10.cell().justify(Justify::Right)],
//!     vec!["Jerry".cell(), 15.cell().justify(Justify::Right)],
//!     vec!["Scooby Doo".cell(), 20.cell().justify(Justify::Right)],
//! ]
//! .table()
//! .bold(true);
//!
//! assert!(print_stdout(table).is_ok());
//! ```
//!
//! Below is the output of the table we created just now:
//!
//! ```markdown
//! +------------+----------------+
//! | Name       | Age (in years) |  <-- This row and all the borders/separators
//! +------------+----------------+      will appear in bold
//! | Tom        |             10 |
//! +------------+----------------+
//! | Jerry      |             15 |
//! +------------+----------------+
//! | Scooby Doo |             25 |
//! +------------+----------------+
//! ```
mod buffers;
mod cell;
mod row;
mod style;
mod table;
#[cfg(any(feature = "title", feature = "derive"))]
mod title;
mod utils;

pub mod format;

pub use termcolor::{Color, ColorChoice};

pub use self::{
    cell::{Cell, CellStruct},
    row::{Row, RowStruct},
    style::Style,
    table::{Table, TableStruct},
};

#[cfg(any(feature = "title", feature = "derive"))]
pub use self::title::{Title, WithTitle};

use std::io::Result;

/// Prints a table to `stdout`
pub fn print_stdout<T: Table>(table: T) -> Result<()> {
    table.table().print_stdout()
}

/// Prints a table to `stderr`
pub fn print_stderr<T: Table>(table: T) -> Result<()> {
    table.table().print_stderr()
}
