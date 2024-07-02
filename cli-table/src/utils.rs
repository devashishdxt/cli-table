use std::io::{Result, Write};

use termcolor::{ColorSpec, WriteColor};

use crate::{
    buffers::Buffers,
    table::{Dimension as TableDimension, HorizontalLine, TableFormat, VerticalLine},
};

const ESC: char = '\x1b';

/// NOTE: `display_width()` is ported from https://docs.rs/ansi-width/0.1.0/src/ansi_width/lib.rs.html#9-55
///
/// Return the display width of a unicode string.
/// This functions takes ANSI-escaped color codes into account.
pub(crate) fn display_width(text: &str) -> usize {
    let mut width = 0;
    let mut chars = text.chars();

    // This lint is a false positive, because we use the iterator later, leading to
    // ownership issues if we follow the lint.
    #[allow(clippy::while_let_on_iterator)]
    while let Some(c) = chars.next() {
        // ESC starts escape sequences, so we need to take characters until the
        // end of the escape sequence.
        if c == ESC {
            let Some(c) = chars.next() else {
                break;
            };
            match c {
                // String terminator character: ends other sequences
                // We probably won't encounter this but it's here for completeness.
                // Or for if we get passed invalid codes.
                '\\' => {
                    // ignore
                }
                // Control Sequence Introducer: continue until `\x40-\x7C`
                '[' => while !matches!(chars.next(), Some('\x40'..='\x7C') | None) {},
                // Operating System Command: continue until ST
                ']' => {
                    let mut last = c;
                    while let Some(new) = chars.next() {
                        if new == '\x07' || (new == '\\' && last == ESC) {
                            break;
                        }
                        last = new;
                    }
                }
                // We don't know what character it is, best bet is to fall back to unicode width
                // The ESC is assumed to have 0 width in this case.
                _ => {
                    width += unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
                }
            }
        } else {
            // If it's a normal character outside an escape sequence, use the
            // unicode width.
            width += unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
        }
    }
    width
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
                        print_char(buffers, line.right_end, color_spec)?;
                    } else {
                        print_str(buffers, "", color_spec)?;
                    }
                }
            }
        }

        println(buffers)?;
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

pub(crate) fn print_char(buffers: &mut Buffers<'_>, c: char, color_spec: &ColorSpec) -> Result<()> {
    buffers.set_color(color_spec)?;
    write!(buffers, "{}", c)?;
    buffers.reset()
}

pub(crate) fn println(buffers: &mut Buffers<'_>) -> Result<()> {
    writeln!(buffers)
}
