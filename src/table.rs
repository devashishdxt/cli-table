use std::io::Result;

use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec};

use crate::{
    buffers::Buffers,
    row::{Dimension as RowDimension, Row, RowStruct},
    style::{Style, StyleStruct},
    utils::*,
};

/// Struct for building a table on command line
pub struct TableStruct {
    /// Rows in the table
    rows: Vec<RowStruct>,
    /// Format of the table
    format: TableFormat,
    /// Style of the table
    style: StyleStruct,
    /// Color preferences for printing the table
    color_choice: ColorChoice,
}

impl TableStruct {
    /// Used to set border of a table
    pub fn border(mut self, border: Border) -> Self {
        self.format.border = border;
        self
    }

    /// Used to set column/row separators of a table
    pub fn separator(mut self, separator: Separator) -> Self {
        self.format.separator = separator;
        self
    }

    /// Used to set the color preferences for printing the table
    pub fn color_choice(mut self, color_choice: ColorChoice) -> Self {
        self.color_choice = color_choice;
        self
    }

    /// Prints current table to `stdout`
    pub(crate) fn print_stdout(&self) -> Result<()> {
        self.print_writer(BufferWriter::stdout(self.color_choice))
    }

    /// Prints current table to `stderr`
    pub(crate) fn print_stderr(&self) -> Result<()> {
        self.print_writer(BufferWriter::stderr(self.color_choice))
    }

    fn color_spec(&self) -> ColorSpec {
        self.style.color_spec()
    }

    fn required_dimension(&self) -> Dimension {
        if self.rows.is_empty() {
            return Default::default();
        }

        let mut heights = Vec::with_capacity(self.rows.len());

        let row_dimension = self.rows[0].required_dimension();

        let mut widths = row_dimension.widths;
        heights.push(row_dimension.height);

        for row in self.rows.iter().skip(1) {
            let row_dimension = row.required_dimension();

            heights.push(row_dimension.height);

            let new_widths = row_dimension.widths;

            for (width, new_width) in widths.iter_mut().zip(new_widths.into_iter()) {
                *width = std::cmp::max(new_width, *width);
            }
        }

        Dimension { widths, heights }
    }

    fn print_writer(&self, writer: BufferWriter) -> Result<()> {
        let table_dimension = self.required_dimension();
        let row_dimensions: Vec<RowDimension> = table_dimension.clone().into();
        let color_spec = self.color_spec();

        print_horizontal_line(
            &writer,
            self.format.border.top.as_ref(),
            &table_dimension,
            &self.format,
            &color_spec,
        )?;

        let mut rows = self.rows.iter().zip(row_dimensions.into_iter()).peekable();

        let mut first = true;

        while let Some((row, row_dimension)) = rows.next() {
            let buffers = row.buffers(&writer, row_dimension)?;

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
                                self.format.separator.title.as_ref(),
                                &table_dimension,
                                &self.format,
                                &color_spec,
                            )?
                        } else {
                            print_horizontal_line(
                                &writer,
                                self.format.separator.row.as_ref(),
                                &table_dimension,
                                &self.format,
                                &color_spec,
                            )?
                        }
                    } else {
                        print_horizontal_line(
                            &writer,
                            self.format.separator.row.as_ref(),
                            &table_dimension,
                            &self.format,
                            &color_spec,
                        )?
                    }

                    first = false;
                }
                None => print_horizontal_line(
                    &writer,
                    self.format.border.bottom.as_ref(),
                    &table_dimension,
                    &self.format,
                    &color_spec,
                )?,
            }
        }

        reset_colors(&writer)
    }
}

/// Trait to convert raw type into table
pub trait Table {
    /// Converts raw type to a table
    fn table(self) -> TableStruct;
}

impl<T, R> Table for T
where
    T: IntoIterator<Item = R>,
    R: Row,
{
    fn table(self) -> TableStruct {
        let rows = self.into_iter().map(Row::row).collect();

        TableStruct {
            rows,
            format: Default::default(),
            style: Default::default(),
            color_choice: ColorChoice::Always,
        }
    }
}

impl Table for TableStruct {
    fn table(self) -> TableStruct {
        self
    }
}

impl Style for TableStruct {
    fn foreground_color(mut self, foreground_color: Option<Color>) -> Self {
        self.style = self.style.foreground_color(foreground_color);
        self
    }

    fn background_color(mut self, background_color: Option<Color>) -> Self {
        self.style = self.style.background_color(background_color);
        self
    }

    fn bold(mut self, bold: bool) -> Self {
        self.style = self.style.bold(bold);
        self
    }

    fn underline(mut self, underline: bool) -> Self {
        self.style = self.style.underline(underline);
        self
    }

    fn italic(mut self, italic: bool) -> Self {
        self.style = self.style.italic(italic);
        self
    }

    fn intense(mut self, intense: bool) -> Self {
        self.style = self.style.intense(intense);
        self
    }

    fn dimmed(mut self, dimmed: bool) -> Self {
        self.style = self.style.dimmed(dimmed);
        self
    }
}

/// A vertical line in a table (border or column separator)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerticalLine {
    pub(crate) filler: char,
}

impl Default for VerticalLine {
    fn default() -> Self {
        Self { filler: '|' }
    }
}

impl VerticalLine {
    /// Creates a new instance of vertical line
    pub fn new(filler: char) -> Self {
        Self { filler }
    }
}

/// A horizontal line in a table (border or row separator)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HorizontalLine {
    pub(crate) left_end: char,
    pub(crate) right_end: char,
    pub(crate) junction: char,
    pub(crate) filler: char,
}

impl Default for HorizontalLine {
    fn default() -> Self {
        Self {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }
    }
}

impl HorizontalLine {
    /// Creates a new instance of horizontal line
    pub fn new(left_end: char, right_end: char, junction: char, filler: char) -> Self {
        Self {
            left_end,
            right_end,
            junction,
            filler,
        }
    }
}

/// Borders of a table
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Border {
    pub(crate) top: Option<HorizontalLine>,
    pub(crate) bottom: Option<HorizontalLine>,
    pub(crate) left: Option<VerticalLine>,
    pub(crate) right: Option<VerticalLine>,
}

impl Border {
    /// Creates a new builder for border
    pub fn builder() -> BorderBuilder {
        BorderBuilder(Border {
            top: None,
            bottom: None,
            left: None,
            right: None,
        })
    }
}

impl Default for Border {
    fn default() -> Self {
        Self {
            top: Some(Default::default()),
            bottom: Some(Default::default()),
            left: Some(Default::default()),
            right: Some(Default::default()),
        }
    }
}

/// Builder for border
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorderBuilder(Border);

impl BorderBuilder {
    /// Set top border of a table
    pub fn top(mut self, top: HorizontalLine) -> Self {
        self.0.top = Some(top);
        self
    }

    /// Set bottom border of a table
    pub fn bottom(mut self, bottom: HorizontalLine) -> Self {
        self.0.bottom = Some(bottom);
        self
    }

    /// Set left border of a table
    pub fn left(mut self, left: VerticalLine) -> Self {
        self.0.left = Some(left);
        self
    }

    /// Set right border of a table
    pub fn right(mut self, right: VerticalLine) -> Self {
        self.0.right = Some(right);
        self
    }

    /// Build border
    pub fn build(self) -> Border {
        self.0
    }
}

/// Inner (column/row) separators of a table
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Separator {
    pub(crate) column: Option<VerticalLine>,
    pub(crate) row: Option<HorizontalLine>,
    pub(crate) title: Option<HorizontalLine>,
}

impl Separator {
    /// Creates a new builder for separator
    pub fn builder() -> SeparatorBuilder {
        SeparatorBuilder(Separator {
            column: None,
            row: None,
            title: None,
        })
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self {
            column: Some(Default::default()),
            row: Some(Default::default()),
            title: None,
        }
    }
}

/// Builder for separator
#[derive(Debug)]
pub struct SeparatorBuilder(Separator);

impl SeparatorBuilder {
    /// Set column separators of a table
    pub fn column(mut self, column: Option<VerticalLine>) -> Self {
        self.0.column = column;
        self
    }

    /// Set column separators of a table
    pub fn row(mut self, row: Option<HorizontalLine>) -> Self {
        self.0.row = row;
        self
    }

    /// Set title of a table
    ///
    /// # None
    ///
    /// When title separator is not preset (i.e., it is `None`), row separator is displayed in place of title separator.
    pub fn title(mut self, title: Option<HorizontalLine>) -> Self {
        self.0.title = title;
        self
    }

    /// Build separator
    pub fn build(self) -> Separator {
        self.0
    }
}

/// Struct for configuring a table's format
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct TableFormat {
    pub(crate) border: Border,
    pub(crate) separator: Separator,
}

/// Dimensions of a table
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub(crate) struct Dimension {
    /// Widths of each column of table
    pub(crate) widths: Vec<usize>,
    /// Height of each row of table
    pub(crate) heights: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_from_str_arr() {
        let table: TableStruct = vec![&["Hello", "World"], &["Scooby", "Doo"]].table();
        assert_eq!(2, table.rows.len());
        assert_eq!(2, table.rows[0].cells.len());
        assert_eq!(2, table.rows[1].cells.len());
    }
}
