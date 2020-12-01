use std::io;

use termcolor::{Buffer, BufferWriter};

use crate::{dimension::RowDimension, Cell};

/// A `Row` in a [`Table`](crate::Table)
pub struct Row {
    cells: Vec<Cell>,
    dimension: Option<RowDimension>,
}

impl Row {
    /// Creates a new [`Row`](crate::Row)
    pub fn new(cells: Vec<Cell>) -> Self {
        Self {
            cells,
            dimension: None,
        }
    }

    /// Returns the number of columns
    pub fn columns(&self) -> usize {
        self.cells.len()
    }

    pub(crate) fn reset(&mut self) {
        self.dimension = None;
        self.cells.iter_mut().for_each(|cell| cell.reset());
    }

    fn init_dimension(&mut self) {
        let mut widths = Vec::with_capacity(self.cells.len());
        let mut height = 0;

        for cell in self.cells.iter_mut() {
            let cell_dimension = cell.dimension();

            widths.push(cell_dimension.width);

            if cell_dimension.height > height {
                height = cell_dimension.height;
            }
        }

        self.dimension = Some(RowDimension { widths, height })
    }

    pub(crate) fn dimension(&mut self) -> RowDimension {
        if self.dimension.is_none() {
            self.init_dimension()
        }

        self.dimension.clone().unwrap()
    }

    fn compute_cell_buffers(
        &mut self,
        writer: &BufferWriter,
        available_dimension: RowDimension,
    ) -> io::Result<Vec<Vec<Buffer>>> {
        self.cells
            .iter_mut()
            .zip(available_dimension.cell_dimensions().into_iter())
            .map(|(cell, dimension)| cell.compute_buffers(writer, dimension))
            .collect()
    }

    pub(crate) fn compute_buffers(
        &mut self,
        writer: &BufferWriter,
        available_dimension: RowDimension,
    ) -> io::Result<Vec<Vec<Buffer>>> {
        let cell_buffers = self.compute_cell_buffers(writer, available_dimension)?;
        Ok(self.zip_buffers(writer, cell_buffers))
    }

    fn zip_buffers(
        &mut self,
        writer: &BufferWriter,
        mut buffers: Vec<Vec<Buffer>>,
    ) -> Vec<Vec<Buffer>> {
        let columns = self.cells.len();
        let dimension = self.dimension();
        let mut zipped_buffers = Vec::with_capacity(dimension.height);

        for i in 0..dimension.height {
            let mut line = Vec::with_capacity(columns);

            for buffer_line in buffers.iter_mut() {
                let mut buffer = writer.buffer();
                std::mem::swap(&mut buffer, &mut buffer_line[i]);
                line.push(buffer);
            }

            zipped_buffers.push(line);
        }

        zipped_buffers
    }
}
