use std::io::{self, Write};

use termcolor::{BufferWriter, Color, ColorSpec, WriteColor};

use crate::{
    format::{HorizontalLine, TableFormat, VerticalLine},
    Error, Row,
};

pub use termcolor::ColorChoice;

const DEFAULT_COLOR_CHOICE: ColorChoice = ColorChoice::Always;

/// Struct for building a [`Table`](crate::Table) on command line
pub struct Table {
    /// Rows in the table
    rows: Vec<Row>,
    /// Format of the table
    format: TableFormat,
    /// The maximum widths of each of the columns in the table
    widths: Vec<usize>,
    /// Color preferences for printing the table
    color_choice: ColorChoice,
}

impl Table {
    /// Creates a new [`Table`](crate::Table)
    pub fn new(rows: Vec<Row>, format: TableFormat) -> Result<Table, Error> {
        validate_equal_columns(&rows)?;
        let widths = get_widths(&rows);

        Ok(Table {
            rows,
            format,
            widths,
            color_choice: DEFAULT_COLOR_CHOICE,
        })
    }

    /// Sent the color color preferences for printing the table
    pub fn set_color_choice(&mut self, color_choice: ColorChoice) {
        self.color_choice = color_choice
    }

    /// Prints current [`Table`](crate::Table) to `stdout`
    pub fn print_stdout(&self) -> io::Result<()> {
        self.print_writer(BufferWriter::stdout(self.color_choice))
    }

    /// Prints current [`Table`](crate::Table) to `stderr`
    pub fn print_stderr(&self) -> io::Result<()> {
        self.print_writer(BufferWriter::stderr(self.color_choice))
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
                    print_char(&writer, ' ', None)?;
                    writer.print(&buffer)?;
                    print_char(&writer, ' ', None)?;

                    match line_buffers.peek() {
                        Some(_) => self
                            .print_vertical_line(&writer, self.format.separator.column.as_ref())?,
                        None => {
                            self.print_vertical_line(&writer, self.format.border.right.as_ref())?
                        }
                    }
                }

                println_str(&writer, "", None)?;
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
                print_char(writer, line.left_end, self.format.foreground_color)?;
            }

            let mut widths = self.widths.iter().peekable();

            while let Some(width) = widths.next() {
                let s = std::iter::repeat(line.filler)
                    .take(width + 2)
                    .collect::<String>();
                print_str(writer, &s, self.format.foreground_color)?;

                match widths.peek() {
                    Some(_) => {
                        if self.format.separator.column.is_some() {
                            print_char(writer, line.junction, self.format.foreground_color)?
                        }
                    }
                    None => {
                        if self.format.border.right.is_some() {
                            println_char(writer, line.right_end, self.format.foreground_color)?;
                        } else {
                            println_str(writer, "", self.format.foreground_color)?;
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
            print_char(writer, line.filler, self.format.foreground_color)?;
        }
        Ok(())
    }
}

fn print_str(writer: &BufferWriter, s: &str, foreground: Option<Color>) -> io::Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    buffer.set_color(&ColorSpec::default().set_fg(foreground))?;
    write!(&mut buffer, "{}", s)?;
    writer.print(&buffer)?;
    Ok(())
}

fn println_str(writer: &BufferWriter, s: &str, foreground: Option<Color>) -> io::Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    buffer.set_color(&ColorSpec::default().set_fg(foreground))?;
    writeln!(&mut buffer, "{}", s)?;
    writer.print(&buffer)?;
    Ok(())
}

fn print_char(writer: &BufferWriter, c: char, foreground: Option<Color>) -> io::Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    buffer.set_color(&ColorSpec::default().set_fg(foreground))?;
    write!(&mut buffer, "{}", c)?;
    writer.print(&buffer)?;
    Ok(())
}

fn println_char(writer: &BufferWriter, c: char, foreground: Option<Color>) -> io::Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    buffer.set_color(&ColorSpec::default().set_fg(foreground))?;
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
