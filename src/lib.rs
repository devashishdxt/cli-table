#![forbid(unsafe_code, unstable_features)]
#![deny(missing_docs)]
//! Rust crate for printing tables on command line.
//!
//! ## Usage
//!
//! Add `cli-table` in your `Cargo.toms`'s `dependencies` section
//!
//! ```toml
//! [dependencies]
//! cli-table = "0.2"
//! ```
//!
//! Each cell in a [`Table`] can be formatted using [`CellFormat`]. [`CellFormat`] can be easily created like this:
//!
//! ```
//! # use cli_table::format::{CellFormat, Justify};
//! // Justifies contents of a cell to right
//! let justify_right = CellFormat::builder().justify(Justify::Right).build();
//!
//! // Makes contents of a cell bold
//! let bold = CellFormat::builder().bold(true).build();
//! ```
//!
//! [`Table`] can be formatted using [`TableFormat`]. It is very easy to create a custom table format, but for
//! simplicity, this crate provides a few predefined [`TableFormat`]s:
//!
//! - [`BORDER_COLUMN_ROW`]: Format with borders, column separators and row separators (calling `Default::default()`
//!   on [`TableFormat`] also returns this format)
//! - [`BORDER_COLUMN_NO_ROW`]: Format with borders and column separators (without row separators)
//! - [`BORDER_COLUMN_TITLE`]: Format with borders, column separators and title separator (without row separators)
//! - [`BORDER_NO_COLUMN_ROW`]: Format with borders and row separators (without column separators)
//! - [`NO_BORDER_COLUMN_ROW`]: Format with no borders, column separators and row separators
//! - [`NO_BORDER_COLUMN_TITLE`]: Format with no borders, column separators and title separator (without row separators)
//!
//! To create a table, you can use [`Table::new()`] like this:
//!
//! ```
//! # use cli_table::format::{CellFormat, Justify};
//! # use cli_table::{Table, Row, Cell};
//! # let justify_right = CellFormat::builder().justify(Justify::Right).build();
//! # let bold = CellFormat::builder().bold(true).build();
//! let table = Table::new(
//!     vec![
//!         Row::new(vec![
//!             Cell::new(&format!("Name"), bold),
//!             Cell::new("Age (in years)", bold),
//!         ]),
//!         Row::new(vec![
//!             Cell::new("Tom", Default::default()),
//!             Cell::new("10", justify_right),
//!         ]),
//!         Row::new(vec![
//!             Cell::new("Jerry", Default::default()),
//!             Cell::new("15", justify_right),
//!         ]),
//!         Row::new(vec![
//!             Cell::new("Scooby Doo", Default::default()),
//!             Cell::new("25", justify_right),
//!         ]),
//!     ],
//!     Default::default(),
//! );
//! ```
//!
//! To print this table on `stdout`, you can call [`table.print_stdout()`].
//!
//! Below is the output of the table we created just now:
//!
//! ```markdown
//! +------------+----------------+
//! | Name       | Age (in years) |  <-- This row will appear in bold
//! +------------+----------------+
//! | Tom        |             10 |
//! +------------+----------------+
//! | Jerry      |             15 |
//! +------------+----------------+
//! | Scooby Doo |             25 |
//! +------------+----------------+
//! ```
//!
//! [`Table`]: struct.Table.html
//! [`CellFormat`]: struct.CellFormat.html
//! [`TableFormat`]: struct.TableFormat.html
//! [`Table::new()`]: struct.Table.html#method.new
//! [`table.print_stdout()`]: struct.Table.html#method.print_stdout
//!
//! [`BORDER_COLUMN_ROW`]: format/constant.BORDER_COLUMN_ROW.html
//! [`BORDER_COLUMN_NO_ROW`]: format/constant.BORDER_COLUMN_NO_ROW.html
//! [`BORDER_COLUMN_TITLE`]: format/constant.BORDER_COLUMN_TITLE.html
//! [`BORDER_NO_COLUMN_ROW`]: format/constant.BORDER_NO_COLUMN_ROW.html
//! [`NO_BORDER_COLUMN_ROW`]: format/constant.NO_BORDER_COLUMN_ROW.html
//! [`NO_BORDER_COLUMN_TITLE`]: format/constant.NO_BORDER_COLUMN_TITLE.html
mod cell;
mod row;
mod table;
mod errors;

pub mod format;

pub use self::cell::Cell;
pub use self::row::Row;
pub use self::table::Table;
