use std::io::{self, Write};

use termcolor::{BufferWriter, ColorChoice, WriteColor};

use crate::{
    format::{HorizontalLine, TableFormat, VerticalLine},
    Error, Row,
};

/// Struct for building a [`Table`](struct.Table.html) on command line
pub struct Table {
    /// Rows in the table
    rows: Vec<Row>,
    /// Format of the table
    format: TableFormat,
    /// The maximum widths of each of the columns in the table
    widths: Vec<usize>,
}

impl Table {
    /// Creates a new [`Table`](struct.Table.html)
    pub fn new(rows: Vec<Row>, format: TableFormat) -> Result<Table, Error> {
        validate_equal_columns(&rows)?;
        let widths = get_widths(&rows);

        Ok(Table {
            rows,
            format,
            widths,
        })
    }

    /// Prints current [`Table`](struct.Table.html) to `stdout`
    pub fn print_stdout(&self) -> io::Result<()> {
        self.print_writer(BufferWriter::stdout(ColorChoice::Always))
    }

    /// Prints current [`Table`](struct.Table.html) to `stderr`
    pub fn print_stderr(&self) -> io::Result<()> {
        self.print_writer(BufferWriter::stderr(ColorChoice::Always))
    }

    fn print_writer(&self, writer: BufferWriter) -> io::Result<()> {
        self.print_horizontal_line(&writer, self.format.border.top.as_ref())?;

        let mut rows = self.rows.iter().peekable();

        let mut first = true;

        while let Some(row) = rows.next() {
            let buffers = row.buffers(&writer, &self.widths)?;

            for line in buffers.into_iter() {
                self.print_vertical_line(&writer, self.format.border.left.as_ref())?;

                let mut line_buffers = line.into_iter().peekable();

                while let Some(buffer) = line_buffers.next() {
                    print_char(&writer, ' ')?;
                    writer.print(&buffer)?;
                    print_char(&writer, ' ')?;

                    match line_buffers.peek() {
                        Some(_) => self
                            .print_vertical_line(&writer, self.format.separator.column.as_ref())?,
                        None => {
                            self.print_vertical_line(&writer, self.format.border.right.as_ref())?
                        }
                    }
                }

                println_str(&writer, "")?;
            }

            match rows.peek() {
                Some(_) => {
                    if first {
                        if self.format.separator.title.is_some() {
                            self.print_horizontal_line(
                                &writer,
                                self.format.separator.title.as_ref(),
                            )?
                        } else {
                            self.print_horizontal_line(&writer, self.format.separator.row.as_ref())?
                        }
                    } else {
                        self.print_horizontal_line(&writer, self.format.separator.row.as_ref())?
                    }

                    first = false;
                }
                None => self.print_horizontal_line(&writer, self.format.border.bottom.as_ref())?,
            }
        }

        Ok(())
    }

    fn print_horizontal_line(
        &self,
        writer: &BufferWriter,
        line: Option<&HorizontalLine>,
    ) -> io::Result<()> {
        if let Some(line) = line {
            if self.format.border.left.is_some() {
                print_char(writer, line.left_end)?;
            }

            let mut widths = self.widths.iter().peekable();

            while let Some(width) = widths.next() {
                let s = std::iter::repeat(line.filler)
                    .take(width + 2)
                    .collect::<String>();
                print_str(writer, &s)?;

                match widths.peek() {
                    Some(_) => {
                        if self.format.separator.column.is_some() {
                            print_char(writer, line.junction)?
                        }
                    }
                    None => {
                        if self.format.border.right.is_some() {
                            println_char(writer, line.right_end)?;
                        } else {
                            println_str(writer, "")?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn print_vertical_line(
        &self,
        writer: &BufferWriter,
        line: Option<&VerticalLine>,
    ) -> io::Result<()> {
        if let Some(line) = line {
            print_char(writer, line.filler)?;
        }
        Ok(())
    }
}

fn print_str(writer: &BufferWriter, s: &str) -> io::Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    write!(&mut buffer, "{}", s)?;
    writer.print(&buffer)?;
    Ok(())
}

fn println_str(writer: &BufferWriter, s: &str) -> io::Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    writeln!(&mut buffer, "{}", s)?;
    writer.print(&buffer)?;
    Ok(())
}

fn print_char(writer: &BufferWriter, c: char) -> io::Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    write!(&mut buffer, "{}", c)?;
    writer.print(&buffer)?;
    Ok(())
}

fn println_char(writer: &BufferWriter, c: char) -> io::Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    writeln!(&mut buffer, "{}", c)?;
    writer.print(&buffer)?;
    Ok(())
}

fn validate_equal_columns(rows: &[Row]) -> Result<(), Error> {
    if rows.len() <= 1 {
        return Ok(());
    }
    let columns = rows[0].columns();

    for row in rows.iter().skip(1) {
        if columns != row.columns() {
            return Err(Error::MismatchedColumns);
        }
    }
    Ok(())
}

/// Determine the max widths for each row in the table.
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
