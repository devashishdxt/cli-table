use std::{
    fmt::Display,
    io::{self, Write},
};

use termcolor::{Buffer, BufferWriter, ColorSpec, WriteColor};
use unicode_width::UnicodeWidthStr;

use crate::{
    dimension::CellDimension,
    format::{Align, CellFormat, Justify},
};

/// A `Cell` in a [`Table`](crate::Table)
pub struct Cell {
    data: Vec<String>,
    format: CellFormat,
    dimension: Option<CellDimension>,
    color_spec: Option<ColorSpec>,
}

/// Return the display width of a unicode string.
/// This functions takes ANSI-escaped color codes into account.
pub fn display_width(text: &str) -> usize {
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

impl Cell {
    /// Creates a new [`Cell`](crate::Cell)
    pub fn new<T: Display + ?Sized>(data: &T, format: CellFormat) -> Self {
        Self {
            data: data.to_string().lines().map(ToString::to_string).collect(),
            format,
            dimension: None,
            color_spec: None,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.dimension = None;
        self.color_spec = None;
    }

    fn init_color_spec(&mut self) {
        self.color_spec = Some(self.format.color_spec());
    }

    fn color_spec(&mut self) -> &ColorSpec {
        if self.color_spec.is_none() {
            self.init_color_spec();
        }

        self.color_spec.as_ref().unwrap()
    }

    fn init_dimension(&mut self) {
        let height = self.data.len() + self.format.padding.top + self.format.padding.bottom;
        let width = self
            .data
            .iter()
            .map(|x| display_width(x))
            .max()
            .unwrap_or_default()
            + self.format.padding.left
            + self.format.padding.right;

        self.dimension = Some(CellDimension { height, width });
    }

    pub(crate) fn dimension(&mut self) -> CellDimension {
        if self.dimension.is_none() {
            self.init_dimension()
        }

        self.dimension.unwrap()
    }

    fn compute_buffer(
        &mut self,
        writer: &BufferWriter,
        available_dimension: CellDimension,
        data: &str,
    ) -> io::Result<Buffer> {
        let dimension = self.dimension();

        let empty_chars = match self.format.justify {
            Justify::Left => self.format.padding.left,
            Justify::Right => {
                (available_dimension.width - dimension.width) + self.format.padding.left
            }
            Justify::Center => {
                ((available_dimension.width - dimension.width) / 2) + self.format.padding.left
            }
        };

        let mut buffer = writer.buffer();
        buffer.set_color(self.color_spec())?;

        for _ in 0..empty_chars {
            write!(buffer, " ")?;
        }

        write!(buffer, "{}", data)?;

        for _ in 0..(available_dimension.width - (display_width(data) + empty_chars)) {
            write!(buffer, " ")?;
        }

        Ok(buffer)
    }

    pub(crate) fn compute_buffers(
        &mut self,
        writer: &BufferWriter,
        available_dimension: CellDimension,
    ) -> io::Result<Vec<Buffer>> {
        let dimension = self.dimension();

        assert!(
            available_dimension.height >= dimension.height,
            "Available height is less than that required by cell"
        );
        assert!(
            available_dimension.width >= dimension.width,
            "Available width is less than that required by cell"
        );

        let mut buffers = Vec::with_capacity(available_dimension.height);
        let blank_lines = match self.format.align {
            Align::Top => self.format.padding.top,
            Align::Bottom => {
                (available_dimension.height - dimension.height) + self.format.padding.top
            }
            Align::Center => {
                ((available_dimension.height - dimension.height) / 2) + self.format.padding.top
            }
        };

        for _ in 0..blank_lines {
            buffers.push(self.compute_buffer(writer, available_dimension, "")?);
        }

        for line in self.data.clone().iter() {
            buffers.push(self.compute_buffer(writer, available_dimension, line)?);
        }

        for _ in 0..(available_dimension.height - (self.data.len() + blank_lines)) {
            buffers.push(self.compute_buffer(writer, available_dimension, "")?);
        }

        Ok(buffers)
    }
}
