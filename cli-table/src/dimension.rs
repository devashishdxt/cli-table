/// Dimensions of a cell
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CellDimension {
    /// Width of a cell
    pub width: usize,
    /// Height of a cell
    pub height: usize,
}

/// Dimensions of a row
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct RowDimension {
    /// Widths of each cell of row
    pub widths: Vec<usize>,
    /// Height of row
    pub height: usize,
}

/// Dimensions of a table
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct TableDimension {
    /// Widths of each column of table
    pub widths: Vec<usize>,
    /// Height of each row of table
    pub heights: Vec<usize>,
}

impl From<RowDimension> for Vec<CellDimension> {
    fn from(row_dimension: RowDimension) -> Self {
        let height = row_dimension.height;

        row_dimension
            .widths
            .into_iter()
            .map(|width| CellDimension { width, height })
            .collect()
    }
}

impl From<TableDimension> for Vec<RowDimension> {
    fn from(table_dimension: TableDimension) -> Self {
        let heights = table_dimension.heights;
        let widths = table_dimension.widths;

        heights
            .into_iter()
            .map(|height| RowDimension {
                widths: widths.clone(),
                height,
            })
            .collect()
    }
}

/// Trait for calculating required dimensions for a type
pub trait RequiredDimension {
    /// Type of dimension for given type
    type Dimension;

    /// Returns the required dimension
    fn required_dimension(&self) -> Option<&Self::Dimension>;

    /// Calculates the required dimension for a type and stores it in the type for future use
    fn set_required_dimension(&mut self);
}

pub trait AvailableDimension: RequiredDimension {
    fn available_dimension(&self) -> Option<&<Self as RequiredDimension>::Dimension>;

    fn set_available_dimension(
        &mut self,
        available_dimension: <Self as RequiredDimension>::Dimension,
    );
}
