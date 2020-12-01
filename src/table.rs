use std::io::{self, Write};

use termcolor::{BufferWriter, ColorSpec, WriteColor};

use crate::{
    dimension::TableDimension,
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
    /// Color preferences for printing the table
    color_choice: ColorChoice,
    /// Dimensions of table
    dimension: Option<TableDimension>,
    /// [`ColorSpec`](termcolor::ColorSpec) for the table
    color_spec: Option<ColorSpec>,
}

impl Table {
    /// Creates a new [`Table`](crate::Table)
    pub fn new(rows: Vec<Row>, format: TableFormat) -> Result<Table, Error> {
        validate_equal_columns(&rows)?;

        Ok(Table {
            rows,
            format,
            color_choice: DEFAULT_COLOR_CHOICE,
            dimension: None,
            color_spec: None,
        })
    }

    /// Sent the color color preferences for printing the table
    pub fn set_color_choice(&mut self, color_choice: ColorChoice) {
        self.color_choice = color_choice
    }

    /// Prints current [`Table`](crate::Table) to `stdout`
    pub fn print_stdout(&mut self) -> io::Result<()> {
        self.print_writer(BufferWriter::stdout(self.color_choice))
    }

    /// Prints current [`Table`](crate::Table) to `stderr`
    pub fn print_stderr(&mut self) -> io::Result<()> {
        self.print_writer(BufferWriter::stderr(self.color_choice))
    }

    /// Resets all the precomputed dimension and color information. This function should only be used when you're
    /// printing table again after making some changes to config or data.
    pub fn reset(&mut self) {
        self.dimension = None;
        self.color_spec = None;

        self.rows.iter_mut().for_each(|row| row.reset());
    }

    fn init_color_spec(&mut self) {
        self.color_spec = Some(self.format.color_spec())
    }

    fn color_spec(&mut self) -> &ColorSpec {
        if self.color_spec.is_none() {
            self.init_color_spec()
        }

        self.color_spec.as_ref().unwrap()
    }

    fn init_dimension(&mut self) {
        if self.rows.is_empty() {
            self.dimension = Some(TableDimension {
                widths: Vec::new(),
                heights: Vec::new(),
            });
            return;
        }

        let mut heights = Vec::with_capacity(self.rows.len());

        let row_dimension = self.rows[0].dimension();

        let mut widths = row_dimension.widths;
        heights.push(row_dimension.height);

        for row in self.rows.iter_mut().skip(1) {
            let row_dimension = row.dimension();

            heights.push(row_dimension.height);

            let new_widths = row_dimension.widths;

            for (width, new_width) in widths.iter_mut().zip(new_widths.into_iter()) {
                *width = std::cmp::max(new_width, *width);
            }
        }

        self.dimension = Some(TableDimension { widths, heights });
    }

    fn dimension(&mut self) -> TableDimension {
        if self.dimension.is_none() {
            self.init_dimension()
        }

        self.dimension.clone().unwrap()
    }

    fn print_writer(&mut self, writer: BufferWriter) -> io::Result<()> {
        let table_dimension = self.dimension();
        let row_dimensions = table_dimension.row_dimensions();
        let table_format = self.format;
        let color_spec = self.color_spec().clone();

        print_horizontal_line(
            &writer,
            self.format.border.top,
            &table_dimension,
            &table_format,
            &color_spec,
        )?;

        let mut rows = self
            .rows
            .iter_mut()
            .zip(row_dimensions.into_iter())
            .peekable();

        let mut first = true;

        while let Some((row, row_dimension)) = rows.next() {
            let buffers = row.compute_buffers(&writer, row_dimension)?;

            for line in buffers.into_iter() {
                print_vertical_line(&writer, self.format.border.left.as_ref(), &color_spec)?;

                let mut line_buffers = line.into_iter().peekable();

                while let Some(buffer) = line_buffers.next() {
                    print_char(&writer, ' ', &color_spec)?;
                    writer.print(&buffer)?;
                    print_char(&writer, ' ', &color_spec)?;

                    match line_buffers.peek() {
                        Some(_) => print_vertical_line(
                            &writer,
                            self.format.separator.column.as_ref(),
                            &color_spec,
                        )?,
                        None => print_vertical_line(
                            &writer,
                            self.format.border.right.as_ref(),
                            &color_spec,
                        )?,
                    }
                }

                println_str(&writer, "", &color_spec)?;
            }

            match rows.peek() {
                Some(_) => {
                    if first {
                        if self.format.separator.title.is_some() {
                            print_horizontal_line(
                                &writer,
                                self.format.separator.title,
                                &table_dimension,
                                &table_format,
                                &color_spec,
                            )?
                        } else {
                            print_horizontal_line(
                                &writer,
                                self.format.separator.row,
                                &table_dimension,
                                &table_format,
                                &color_spec,
                            )?
                        }
                    } else {
                        print_horizontal_line(
                            &writer,
                            self.format.separator.row,
                            &table_dimension,
                            &table_format,
                            &color_spec,
                        )?
                    }

                    first = false;
                }
                None => print_horizontal_line(
                    &writer,
                    self.format.border.bottom,
                    &table_dimension,
                    &table_format,
                    &color_spec,
                )?,
            }
        }

        reset_colors(&writer)
    }
}

fn print_horizontal_line(
    writer: &BufferWriter,
    line: Option<HorizontalLine>,
    dimension: &TableDimension,
    table_format: &TableFormat,
    color_spec: &ColorSpec,
) -> io::Result<()> {
    if let Some(line) = line {
        if table_format.border.left.is_some() {
            print_char(writer, line.left_end, color_spec)?;
        }

        let mut widths = dimension.widths.iter().peekable();

        while let Some(width) = widths.next() {
            let s = std::iter::repeat(line.filler)
                .take(width + 2)
                .collect::<String>();
            print_str(writer, &s, color_spec)?;

            match widths.peek() {
                Some(_) => {
                    if table_format.separator.column.is_some() {
                        print_char(writer, line.junction, color_spec)?
                    }
                }
                None => {
                    if table_format.border.right.is_some() {
                        println_char(writer, line.right_end, color_spec)?;
                    } else {
                        println_str(writer, "", color_spec)?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn print_vertical_line(
    writer: &BufferWriter,
    line: Option<&VerticalLine>,
    color_spec: &ColorSpec,
) -> io::Result<()> {
    if let Some(line) = line {
        print_char(writer, line.filler, color_spec)?;
    }
    Ok(())
}

fn print_str(writer: &BufferWriter, s: &str, color_spec: &ColorSpec) -> io::Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    buffer.set_color(color_spec)?;
    write!(&mut buffer, "{}", s)?;
    writer.print(&buffer)?;
    Ok(())
}

fn println_str(writer: &BufferWriter, s: &str, color_spec: &ColorSpec) -> io::Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    buffer.set_color(color_spec)?;
    writeln!(&mut buffer, "{}", s)?;
    writer.print(&buffer)?;
    Ok(())
}

fn print_char(writer: &BufferWriter, c: char, color_spec: &ColorSpec) -> io::Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    buffer.set_color(color_spec)?;
    write!(&mut buffer, "{}", c)?;
    writer.print(&buffer)?;
    Ok(())
}

fn println_char(writer: &BufferWriter, c: char, color_spec: &ColorSpec) -> io::Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    buffer.set_color(color_spec)?;
    writeln!(&mut buffer, "{}", c)?;
    writer.print(&buffer)?;
    Ok(())
}

fn reset_colors(writer: &BufferWriter) -> io::Result<()> {
    let mut buffer = writer.buffer();
    buffer.reset()?;
    write!(&mut buffer, "")?;
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
