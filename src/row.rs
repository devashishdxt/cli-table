use std::io;

use termcolor::{Buffer, BufferWriter};

use crate::Cell;

/// A `Row` in a [`Table`](struct.Table.html)
pub struct Row {
    pub(crate) cells: Vec<Cell>,
    pub(crate) height: usize,
}

impl Row {
    /// Creates a new [`Row`](struct.Row.html)
    pub fn new(cells: Vec<Cell>) -> Self {
        let height = cells
            .iter()
            .map(|cell| cell.height)
            .max()
            .unwrap_or_default();

        Self { cells, height }
    }

    #[inline]
    pub(crate) fn columns(&self) -> usize {
        self.cells.len()
    }

    #[inline]
    pub(crate) fn widths(&self) -> Vec<usize> {
        self.cells.iter().map(|cell| cell.width).collect()
    }

    pub(crate) fn buffers(
        &self,
        writer: &BufferWriter,
        widths: &[usize],
    ) -> io::Result<Vec<Vec<Buffer>>> {
        let buffers = self
            .cells
            .iter()
            .zip(widths.iter())
            .map(|(cell, width)| cell.buffers(writer, self.height, *width))
            .collect::<io::Result<Vec<Vec<Option<Buffer>>>>>()?;
        Ok(self.zip_buffers(buffers))
    }

    fn zip_buffers(&self, mut buffers: Vec<Vec<Option<Buffer>>>) -> Vec<Vec<Buffer>> {
        let columns = self.cells.len();
        let mut zipped_buffers = Vec::with_capacity(self.height);

        for i in 0..self.height {
            let mut line = Vec::with_capacity(columns);

            for buffer_line in buffers.iter_mut() {
                let mut buffer = None;
                std::mem::swap(&mut buffer, &mut buffer_line[i]);
                line.push(
                    buffer.expect("Expected a buffer at given height and column. This is a bug!"),
                );
            }

            zipped_buffers.push(line);
        }

        zipped_buffers
    }
}
