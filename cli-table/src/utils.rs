use std::io::{Result, Write};

use termcolor::{ColorSpec, WriteColor};
use unicode_width::UnicodeWidthStr;

use crate::{
    buffers::Buffers,
    table::{Dimension as TableDimension, HorizontalLine, TableFormat, VerticalLine},
};

/// NOTE: `display_width()` is ported from https://github.com/phsym/prettytable-rs
///
/// Return the display width of a unicode string.
/// This functions takes ANSI-escaped color codes into account.
pub(crate) fn display_width(text: &str) -> usize {
    let width = UnicodeWidthStr::width(text);

    let mut state = 0;
    let mut hidden = 0;

    for c in text.chars() {
        state = match (state, c) {
            (0, '\u{1b}') => 1,
            (1, '[') => 2,
            (1, _) => 0,
            (2, 'm') => 3,
            _ => state,
        };

        // We don't count escape characters as hidden as
        // UnicodeWidthStr::width already considers them.
        if state > 1 {
            hidden += 1;
        }

        if state == 3 {
            state = 0;
        }
    }

    width - hidden
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() || v[0].is_empty() {
        return v;
    }

    let columns = v.len();
    let height = v[0].len();

    let mut optional_v: Vec<Vec<Option<T>>> = v
        .into_iter()
        .map(|cell_v| cell_v.into_iter().map(Some).collect())
        .collect();

    let mut transpose = Vec::with_capacity(height);

    for i in 0..height {
        let mut row_buffer = Vec::with_capacity(columns);

        for inner_v in optional_v.iter_mut().take(columns) {
            row_buffer.push(std::mem::take(&mut inner_v[i]).unwrap());
        }

        transpose.push(row_buffer);
    }

    transpose
}

pub(crate) fn print_horizontal_line(
    buffers: &mut Buffers<'_>,
    line: Option<&HorizontalLine>,
    table_dimension: &TableDimension,
    table_format: &TableFormat,
    color_spec: &ColorSpec,
) -> Result<()> {
    if let Some(line) = line {
        if table_format.border.left.is_some() {
            print_char(buffers, line.left_end, color_spec)?;
        }

        let mut widths = table_dimension.widths.iter().peekable();

        while let Some(width) = widths.next() {
            let s = std::iter::repeat(line.filler)
                .take(width + 2)
                .collect::<String>();
            print_str(buffers, &s, color_spec)?;

            match widths.peek() {
                Some(_) => {
                    if table_format.separator.column.is_some() {
                        print_char(buffers, line.junction, color_spec)?
                    }
                }
                None => {
                    if table_format.border.right.is_some() {
                        println_char(buffers, line.right_end, color_spec)?;
                    } else {
                        println_str(buffers, "", color_spec)?;
                    }
                }
            }
        }
    }

    Ok(())
}

pub(crate) fn print_vertical_line(
    buffers: &mut Buffers<'_>,
    line: Option<&VerticalLine>,
    color_spec: &ColorSpec,
) -> Result<()> {
    if let Some(line) = line {
        print_char(buffers, line.filler, color_spec)?;
    }
    Ok(())
}

pub(crate) fn print_str(buffers: &mut Buffers<'_>, s: &str, color_spec: &ColorSpec) -> Result<()> {
    buffers.set_color(color_spec)?;
    write!(buffers, "{}", s)?;
    buffers.reset()
}

pub(crate) fn println_str(
    buffers: &mut Buffers<'_>,
    s: &str,
    color_spec: &ColorSpec,
) -> Result<()> {
    buffers.set_color(color_spec)?;
    writeln!(buffers, "{}", s)?;
    buffers.reset()
}

pub(crate) fn print_char(buffers: &mut Buffers<'_>, c: char, color_spec: &ColorSpec) -> Result<()> {
    buffers.set_color(color_spec)?;
    write!(buffers, "{}", c)?;
    buffers.reset()
}

pub(crate) fn println_char(
    buffers: &mut Buffers<'_>,
    c: char,
    color_spec: &ColorSpec,
) -> Result<()> {
    buffers.set_color(color_spec)?;
    writeln!(buffers, "{}", c)?;
    buffers.reset()
}
