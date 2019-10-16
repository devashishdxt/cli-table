pub use termcolor::Color;

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
    #[inline]
    pub fn builder() -> CellFormatBuilder {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct CellFormatBuilder(CellFormat);

impl CellFormatBuilder {
    #[inline]
    pub fn justify(mut self, justify: Justify) -> Self {
        self.0.justify = justify;
        self
    }

    #[inline]
    pub fn align(mut self, align: Align) -> Self {
        self.0.align = align;
        self
    }

    #[inline]
    pub fn foreground_color(mut self, foreground_color: Option<Color>) -> Self {
        self.0.foreground_color = foreground_color;
        self
    }

    #[inline]
    pub fn background_color(mut self, background_color: Option<Color>) -> Self {
        self.0.background_color = background_color;
        self
    }

    #[inline]
    pub fn bold(mut self, bold: bool) -> Self {
        self.0.bold = bold;
        self
    }

    #[inline]
    pub fn underline(mut self, underline: bool) -> Self {
        self.0.underline = underline;
        self
    }

    #[inline]
    pub fn build(self) -> CellFormat {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Justify {
    Left,
    Right,
    Center,
}

impl Default for Justify {
    #[inline]
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Align {
    Top,
    Bottom,
    Center,
}

impl Default for Align {
    #[inline]
    fn default() -> Self {
        Self::Top
    }
}
