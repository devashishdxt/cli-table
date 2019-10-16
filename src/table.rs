use std::io::{Result, Write};

use termcolor::{BufferWriter, ColorChoice, WriteColor};

use crate::Row;

/// Struct for building a `Table` on command line
pub struct Table {
    rows: Vec<Row>,
    widths: Vec<usize>,
}

impl Table {
    /// Creates a new [`Table`](struct.Table.html)
    pub fn new(rows: Vec<Row>) -> Table {
        validate_equal_columns(&rows);
        let widths = get_widths(&rows);

        Table { rows, widths }
    }

    /// Prints current [`Table`](struct.Table.html) to `stdout`
    pub fn print_std(&self) -> Result<()> {
        let writer = BufferWriter::stdout(ColorChoice::Always);

        print_horizontal_separator(&writer, &self.widths)?;

        for row in self.rows.iter() {
            let buffers = row.buffers(&writer, &self.widths)?;

            for line in buffers.into_iter() {
                print_str(&writer, "|")?;
                for buffer in line.into_iter() {
                    print_str(&writer, " ")?;
                    writer.print(&buffer)?;
                    print_str(&writer, " |")?;
                }
                println_str(&writer, "")?;
            }

            print_horizontal_separator(&writer, &self.widths)?;
        }

        Ok(())
    }
}

fn print_str(writer: &BufferWriter, s: &str) -> Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    write!(&mut buffer, "{}", s)?;
    writer.print(&buffer)?;
    Ok(())
}

fn println_str(writer: &BufferWriter, s: &str) -> Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    writeln!(&mut buffer, "{}", s)?;
    writer.print(&buffer)?;
    Ok(())
}

fn print_horizontal_separator(writer: &BufferWriter, widths: &[usize]) -> Result<()> {
    print_str(writer, "+")?;

    for width in widths.iter() {
        let s = std::iter::repeat('-').take(width + 2).collect::<String>();
        print_str(writer, &s)?;
        print_str(writer, "+")?;
    }

    println_str(writer, "")?;

    Ok(())
}

fn validate_equal_columns(rows: &[Row]) {
    if rows.len() <= 1 {
        return;
    }
    let columns = rows[0].columns();

    for row in rows.iter().skip(1) {
        if columns != row.columns() {
            panic!("Mismatch column numbers in different rows");
        }
    }
}

fn get_widths(rows: &[Row]) -> Vec<usize> {
    if rows.is_empty() {
        return Vec::new();
    }

    let mut widths = rows[0].widths();

    for row in rows.iter().skip(1) {
        let new_widths = row.widths();

        for (width, new_width) in widths.iter_mut().zip(new_widths.into_iter()) {
            *width = std::cmp::max(new_width, *width);
        }
    }

    widths
}
