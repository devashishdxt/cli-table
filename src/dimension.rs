/// Dimensions of a cell
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CellDimension {
    /// Width of cell
    pub width: usize,
    /// Height of cell
    pub height: usize,
}

/// Dimensions of a row
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RowDimension {
    /// Widths of each cell of row
    pub widths: Vec<usize>,
    /// Height of row
    pub height: usize,
}

impl RowDimension {
    pub fn cell_dimensions(self) -> Vec<CellDimension> {
        let height = self.height;

        self.widths
            .into_iter()
            .map(|width| CellDimension {
                width,
                height: height,
            })
            .collect()
    }
}

/// Dimensions of a table
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TableDimension {
    /// Widths of each column of table
    pub widths: Vec<usize>,
    /// Height of each row of table
    pub heights: Vec<usize>,
}

impl TableDimension {
    pub fn row_dimensions(&self) -> Vec<RowDimension> {
        self.heights
            .iter()
            .map(|height| RowDimension {
                widths: self.widths.clone(),
                height: *height,
            })
            .collect()
    }
}
