//! Utilities for formatting of a [`Table`](struct.Table.html)
mod cell;
mod table;

pub use self::cell::{
    Align, CellFormat, CellFormatBuilder, Color, Justify, Padding, PaddingBuilder,
};
pub use self::table::{
    Border, BorderBuilder, HorizontalLine, Separator, SeparatorBuilder, TableFormat, VerticalLine,
};

/// Format with borders, column separators and row separators (calling `Default::default()` on [`TableFormat`](struct.TableFormat.html)
/// also returns this format)
///
/// ```markdown
/// +------------+----------------+
/// | Name       | Age (in years) |
/// +------------+----------------+
/// | Tom        |             10 |
/// +------------+----------------+
/// | Jerry      |             15 |
/// +------------+----------------+
/// | Scooby Doo |             25 |
/// +------------+----------------+
/// ```
pub const BORDER_COLUMN_ROW: TableFormat = TableFormat {
    foreground: None,
    border: Border {
        top: Some(HorizontalLine {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }),
        bottom: Some(HorizontalLine {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }),
        left: Some(VerticalLine { filler: '|' }),
        right: Some(VerticalLine { filler: '|' }),
    },
    separator: Separator {
        column: Some(VerticalLine { filler: '|' }),
        row: Some(HorizontalLine {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }),
        title: None,
    },
};

/// Format with borders, column separators and title separator (without row separators)
///
/// ```markdown
/// +------------+----------------+
/// | Name       | Age (in years) |
/// +------------+----------------+
/// | Tom        |             10 |
/// | Jerry      |             15 |
/// | Scooby Doo |             25 |
/// +------------+----------------+
/// ```
pub const BORDER_COLUMN_TITLE: TableFormat = TableFormat {
    foreground: None,
    border: Border {
        top: Some(HorizontalLine {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }),
        bottom: Some(HorizontalLine {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }),
        left: Some(VerticalLine { filler: '|' }),
        right: Some(VerticalLine { filler: '|' }),
    },
    separator: Separator {
        column: Some(VerticalLine { filler: '|' }),
        row: None,
        title: Some(HorizontalLine {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }),
    },
};

/// Format with borders and column separators (without row separators)
///
/// ```markdown
/// +------------+----------------+
/// | Name       | Age (in years) |
/// | Tom        |             10 |
/// | Jerry      |             15 |
/// | Scooby Doo |             25 |
/// +------------+----------------+
/// ```
pub const BORDER_COLUMN_NO_ROW: TableFormat = TableFormat {
    foreground: None,
    border: Border {
        top: Some(HorizontalLine {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }),
        bottom: Some(HorizontalLine {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }),
        left: Some(VerticalLine { filler: '|' }),
        right: Some(VerticalLine { filler: '|' }),
    },
    separator: Separator {
        column: Some(VerticalLine { filler: '|' }),
        row: None,
        title: None,
    },
};

/// Format with no borders, column separators and title separator (without row separators)
///
/// ```markdown
///  Name       | Age (in years)
/// ------------+----------------
///  Tom        |             10
///  Jerry      |             15
///  Scooby Doo |             25
/// ```
pub const NO_BORDER_COLUMN_TITLE: TableFormat = TableFormat {
    foreground: None,
    border: Border {
        top: None,
        bottom: None,
        left: None,
        right: None,
    },
    separator: Separator {
        column: Some(VerticalLine { filler: '|' }),
        row: None,
        title: Some(HorizontalLine {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }),
    },
};

/// Format with no borders, column separators and row separators
///
/// ```markdown
///  Name       | Age (in years)
/// ------------+----------------
///  Tom        |             10
/// ------------+----------------
///  Jerry      |             15
/// ------------+----------------
///  Scooby Doo |             25
/// ```
pub const NO_BORDER_COLUMN_ROW: TableFormat = TableFormat {
    foreground: None,
    border: Border {
        top: None,
        bottom: None,
        left: None,
        right: None,
    },
    separator: Separator {
        column: Some(VerticalLine { filler: '|' }),
        row: Some(HorizontalLine {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }),
        title: None,
    },
};

/// Format with borders and row separators (without column separators)
///
/// ```markdown
/// +----------------------------+
/// | Name        Age (in years) |
/// +----------------------------+
/// | Tom                     10 |
/// +----------------------------+
/// | Jerry                   15 |
/// +----------------------------+
/// | Scooby Doo              25 |
/// +----------------------------+
/// ```
pub const BORDER_NO_COLUMN_ROW: TableFormat = TableFormat {
    foreground: None,
    border: Border {
        top: Some(HorizontalLine {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }),
        bottom: Some(HorizontalLine {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }),
        left: Some(VerticalLine { filler: '|' }),
        right: Some(VerticalLine { filler: '|' }),
    },
    separator: Separator {
        column: None,
        row: Some(HorizontalLine {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }),
        title: None,
    },
};
