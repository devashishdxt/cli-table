use std::{
    fmt::Display,
    io::{Result, Write},
};

use termcolor::{Buffer, BufferWriter, ColorSpec, WriteColor};
use unicode_width::UnicodeWidthStr;

use crate::format::{Align, CellFormat, Justify};

/// A `Cell` in a [`Table`](struct.Table.html)
#[derive(Debug)]
pub struct Cell {
    pub(crate) data: Vec<String>,
    pub(crate) format: CellFormat,
    pub(crate) height: usize,
    pub(crate) width: usize,
}

impl Cell {
    /// Creates a new [`Cell`](struct.Cell.html)
    pub fn new<T: Display + ?Sized>(data: &T, format: CellFormat) -> Self {
        let data: Vec<String> = data.to_string().lines().map(ToString::to_string).collect();
        let height = data.len();
        let width = data.iter().map(|x| x.width()).max().unwrap_or_default();

        Self {
            data,
            format,
            height,
            width,
        }
    }

    pub(crate) fn buffers(
        &self,
        writer: &BufferWriter,
        height: usize,
        width: usize,
    ) -> Result<Vec<Option<Buffer>>> {
        assert!(
            height >= self.height,
            "Provided height is less than that required by cell"
        );
        assert!(
            width >= self.width,
            "Provided width is less than that required by cell"
        );

        let color_spec = self.color_spec();

        let mut buffers = Vec::with_capacity(height);
        let blank_lines = match self.format.align {
            Align::Top => 0,
            Align::Bottom => (height - self.height),
            Align::Center => (height - self.height) / 2,
        };

        for _ in 0..blank_lines {
            buffers.push(Some(get_buffer(
                writer,
                "",
                width,
                self.format.justify,
                &color_spec,
            )?));
        }

        for line in self.data.iter() {
            buffers.push(Some(get_buffer(
                writer,
                line,
                width,
                self.format.justify,
                &color_spec,
            )?));
        }

        for _ in 0..(height - (self.height + blank_lines)) {
            buffers.push(Some(get_buffer(
                writer,
                "",
                width,
                self.format.justify,
                &color_spec,
            )?));
        }

        Ok(buffers)
    }

    fn color_spec(&self) -> ColorSpec {
        let mut spec = ColorSpec::new();

        spec.set_fg(self.format.foreground_color);
        spec.set_bg(self.format.background_color);
        spec.set_bold(self.format.bold);
        spec.set_underline(self.format.underline);

        spec
    }
}

fn get_buffer(
    writer: &BufferWriter,
    data: &str,
    width: usize,
    justify: Justify,
    color_spec: &ColorSpec,
) -> Result<Buffer> {
    let mut buffer = writer.buffer();
    buffer.set_color(&color_spec)?;

    match justify {
        Justify::Left => write!(&mut buffer, "{:<width$}", data, width = width)?,
        Justify::Right => write!(&mut buffer, "{:>width$}", data, width = width)?,
        Justify::Center => write!(&mut buffer, "{:^width$}", data, width = width)?,
    }

    Ok(buffer)
}
