#[derive(Debug, Clone, Copy)]
pub struct TableFormat {
    has_horizontal_borders: bool,
    has_vertical_borders: bool,
    has_row_separators: bool,
    has_column_separators: bool,
}

impl TableFormat {
    #[inline]
    pub fn builder() -> TableFormatBuilder {
        Default::default()
    }
}

impl Default for TableFormat {
    #[inline]
    fn default() -> TableFormat {
        TableFormat {
            has_horizontal_borders: true,
            has_vertical_borders: true,
            has_row_separators: true,
            has_column_separators: true,
        }
    }
}

#[derive(Debug, Default)]
pub struct TableFormatBuilder(TableFormat);

impl TableFormatBuilder {
    #[inline]
    pub fn has_horizontal_borders(&mut self, has_horizontal_borders: bool) -> &mut Self {
        self.0.has_horizontal_borders = has_horizontal_borders;
        self
    }

    #[inline]
    pub fn has_vertical_borders(&mut self, has_vertical_borders: bool) -> &mut Self {
        self.0.has_vertical_borders = has_vertical_borders;
        self
    }

    #[inline]
    pub fn has_column_separators(&mut self, has_column_separators: bool) -> &mut Self {
        self.0.has_column_separators = has_column_separators;
        self
    }

    #[inline]
    pub fn has_row_separators(&mut self, has_row_separators: bool) -> &mut Self {
        self.0.has_row_separators = has_row_separators;
        self
    }

    #[inline]
    pub fn build(self) -> TableFormat {
        self.0
    }
}
