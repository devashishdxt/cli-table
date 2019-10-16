pub use termcolor::Color;

/// Struct for configuring a [`Cell`](struct.Cell.html)'s format
#[derive(Debug, Clone, Copy)]
pub struct CellFormat {
    pub(crate) justify: Justify,
    pub(crate) align: Align,
    pub(crate) foreground_color: Option<Color>,
    pub(crate) background_color: Option<Color>,
    pub(crate) bold: bool,
    pub(crate) underline: bool,
}

impl Default for CellFormat {
    fn default() -> Self {
        Self {
            justify: Default::default(),
            align: Default::default(),
            foreground_color: None,
            background_color: None,
            bold: false,
            underline: false,
        }
    }
}

impl CellFormat {
    /// Creates a new builder for [`CellFormat`](struct.CellFormat.html)
    #[inline]
    pub fn builder() -> CellFormatBuilder {
        Default::default()
    }
}

/// Builder for [`CellFormat`](struct.CellFormat.html)
#[derive(Debug, Default)]
pub struct CellFormatBuilder(CellFormat);

impl CellFormatBuilder {
    /// Justify contents of a [`Cell`](struct.Cell.html)
    #[inline]
    pub fn justify(mut self, justify: Justify) -> Self {
        self.0.justify = justify;
        self
    }

    /// Align contents of a [`Cell`](struct.Cell.html)
    #[inline]
    pub fn align(mut self, align: Align) -> Self {
        self.0.align = align;
        self
    }

    /// Set foreground color of a [`Cell`](struct.Cell.html)
    #[inline]
    pub fn foreground_color(mut self, foreground_color: Option<Color>) -> Self {
        self.0.foreground_color = foreground_color;
        self
    }

    /// Set background color of a [`Cell`](struct.Cell.html)
    #[inline]
    pub fn background_color(mut self, background_color: Option<Color>) -> Self {
        self.0.background_color = background_color;
        self
    }

    /// Set contents of [`Cell`](struct.Cell.html) to be bold
    #[inline]
    pub fn bold(mut self, bold: bool) -> Self {
        self.0.bold = bold;
        self
    }

    /// Set contents of [`Cell`](struct.Cell.html) to be underlined
    #[inline]
    pub fn underline(mut self, underline: bool) -> Self {
        self.0.underline = underline;
        self
    }

    /// Build [`CellFormat`](struct.CellFormat.html)
    #[inline]
    pub fn build(self) -> CellFormat {
        self.0
    }
}

/// Used to horizontally justify contents of a [`Cell`](struct.Cell.html)
#[derive(Debug, Clone, Copy)]
pub enum Justify {
    /// Justifies contents to left
    Left,
    /// Justifies contents to right
    Right,
    /// Justifies contents to center
    Center,
}

impl Default for Justify {
    #[inline]
    fn default() -> Self {
        Self::Left
    }
}

/// Used to vertically align contents of a [`Cell`](struct.Cell.html)
#[derive(Debug, Clone, Copy)]
pub enum Align {
    /// Aligns contents to top
    Top,
    /// Aligns contents to bottom
    Bottom,
    /// Aligns contents to center
    Center,
}

impl Default for Align {
    #[inline]
    fn default() -> Self {
        Self::Top
    }
}
