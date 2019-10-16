#![forbid(unsafe_code, unstable_features)]
#![deny(missing_docs)]
//! A crate for printing tables on command line
//!
//! # Usage
//!
//! ```
//! use std::io::Result;
//!
//! use cli_table::{Cell, CellFormat, Justify, Row, Table};
//!
//! fn main() -> Result<()> {
//!     let justify_right = CellFormat::builder().justify(Justify::Right).build();
//!     let bold = CellFormat::builder().bold(true).build();
//!
//!     let table = Table::new(vec![
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
//!     ]);
//!
//!     table.print_std()
//! }
//! ```
mod cell;
mod format;
mod row;
mod table;

pub use self::cell::Cell;
pub use self::format::{Align, CellFormat, CellFormatBuilder, Color, Justify};
pub use self::row::Row;
pub use self::table::Table;
