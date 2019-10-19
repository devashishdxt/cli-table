//! Utilities for formatting of a [`Table`](struct.Table.html)
mod cell;
mod table;

pub use self::cell::{Align, CellFormat, CellFormatBuilder, Color, Justify};
pub use self::table::{
    Border, BorderBuilder, HorizontalLine, Separator, SeparatorBuilder, TableFormat, VerticalLine,
};
