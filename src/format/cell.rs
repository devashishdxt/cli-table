pub use termcolor::Color;

/// Struct for configuring a [`Cell`](struct.Cell.html)'s format
#[derive(Debug, Clone, Copy, Default)]
pub struct CellFormat {
    pub(crate) justify: Justify,
    pub(crate) align: Align,
    pub(crate) padding: Padding,
    pub(crate) foreground_color: Option<Color>,
    pub(crate) background_color: Option<Color>,
    pub(crate) bold: bool,
    pub(crate) underline: bool,
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

    /// Sets padding of a [`Cell`](struct.Cell.html)
    #[inline]
    pub fn padding(mut self, padding: Padding) -> Self {
        self.0.padding = padding;
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

/// Used to add padding to the contents of a [`Cell`](struct.Cell.html)
#[derive(Debug, Clone, Copy, Default)]
pub struct Padding {
    /// Left padding
    pub(crate) left: usize,
    /// Right padding
    pub(crate) right: usize,
    /// Top padding
    pub(crate) top: usize,
    /// Bottom padding
    pub(crate) bottom: usize,
}

impl Padding {
    /// Creates a new builder for [`Padding`](struct.Padding.html)
    pub fn builder() -> PaddingBuilder {
        Default::default()
    }
}

/// Builder for [`Padding`](struct.Padding.html)
#[derive(Debug, Default)]
pub struct PaddingBuilder(Padding);

impl PaddingBuilder {
    /// Sets left padding of a [`Cell`](struct.Cell.html)
    #[inline]
    pub fn left(mut self, left_padding: usize) -> Self {
        self.0.left = left_padding;
        self
    }

    /// Sets right padding of a [`Cell`](struct.Cell.html)
    #[inline]
    pub fn right(mut self, right_padding: usize) -> Self {
        self.0.right = right_padding;
        self
    }

    /// Sets top padding of a [`Cell`](struct.Cell.html)
    #[inline]
    pub fn top(mut self, top_padding: usize) -> Self {
        self.0.top = top_padding;
        self
    }

    /// Sets bottom padding of a [`Cell`](struct.Cell.html)
    #[inline]
    pub fn bottom(mut self, bottom_padding: usize) -> Self {
        self.0.bottom = bottom_padding;
        self
    }

    /// Build [`Padding`](struct.Padding.html)
    #[inline]
    pub fn build(self) -> Padding {
        self.0
    }
}
