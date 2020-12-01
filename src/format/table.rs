use termcolor::{Color, ColorSpec};

/// A vertical line in a [`Table`](crate::Table) (border or column separator)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerticalLine {
    pub(crate) filler: char,
}

impl Default for VerticalLine {
    fn default() -> Self {
        Self { filler: '|' }
    }
}

impl VerticalLine {
    /// Creates a new instance of [`VerticalLine`](crate::format::VerticalLine)
    pub fn new(filler: char) -> Self {
        Self { filler }
    }
}

/// A horizontal line in a [`Table`](crate::Table) (border or row separator)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HorizontalLine {
    pub(crate) left_end: char,
    pub(crate) right_end: char,
    pub(crate) junction: char,
    pub(crate) filler: char,
}

impl Default for HorizontalLine {
    fn default() -> Self {
        Self {
            left_end: '+',
            right_end: '+',
            junction: '+',
            filler: '-',
        }
    }
}

impl HorizontalLine {
    /// Creates a new instance of [`HorizontalLine`](crate::format::HorizontalLine)
    pub fn new(left_end: char, right_end: char, junction: char, filler: char) -> Self {
        Self {
            left_end,
            right_end,
            junction,
            filler,
        }
    }
}

/// Borders of a [`Table`](crate::Table)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Border {
    pub(crate) top: Option<HorizontalLine>,
    pub(crate) bottom: Option<HorizontalLine>,
    pub(crate) left: Option<VerticalLine>,
    pub(crate) right: Option<VerticalLine>,
}

impl Border {
    /// Creates a new builder for [`Border`](crate::format::Border)
    pub fn builder() -> BorderBuilder {
        BorderBuilder(Border {
            top: None,
            bottom: None,
            left: None,
            right: None,
        })
    }
}

impl Default for Border {
    fn default() -> Self {
        Self {
            top: Some(Default::default()),
            bottom: Some(Default::default()),
            left: Some(Default::default()),
            right: Some(Default::default()),
        }
    }
}

/// Builder for [`Border`](crate::format::Border)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorderBuilder(Border);

impl BorderBuilder {
    /// Set top border of a [`Table`](crate::Table)
    pub fn top(mut self, top: HorizontalLine) -> Self {
        self.0.top = Some(top);
        self
    }

    /// Set bottom border of a [`Table`](crate::Table)
    pub fn bottom(mut self, bottom: HorizontalLine) -> Self {
        self.0.bottom = Some(bottom);
        self
    }

    /// Set left border of a [`Table`](crate::Table)
    pub fn left(mut self, left: VerticalLine) -> Self {
        self.0.left = Some(left);
        self
    }

    /// Set right border of a [`Table`](crate::Table)
    pub fn right(mut self, right: VerticalLine) -> Self {
        self.0.right = Some(right);
        self
    }

    /// Build [`Border`](crate::format::Border)
    pub fn build(self) -> Border {
        self.0
    }
}

/// Inner (column/row) separators of a [`Table`](crate::Table)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Separator {
    pub(crate) column: Option<VerticalLine>,
    pub(crate) row: Option<HorizontalLine>,
    pub(crate) title: Option<HorizontalLine>,
}

impl Separator {
    /// Creates a new builder for [`Separator`](crate::format::Separator)
    pub fn builder() -> SeparatorBuilder {
        SeparatorBuilder(Separator {
            column: None,
            row: None,
            title: None,
        })
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self {
            column: Some(Default::default()),
            row: Some(Default::default()),
            title: None,
        }
    }
}

/// Builder for [`Separator`](crate::format::Separator)
#[derive(Debug)]
pub struct SeparatorBuilder(Separator);

impl SeparatorBuilder {
    /// Set column separators of a [`Table`](crate::Table)
    pub fn column(mut self, column: Option<VerticalLine>) -> Self {
        self.0.column = column;
        self
    }

    /// Set column separators of a [`Table`](crate::Table)
    pub fn row(mut self, row: Option<HorizontalLine>) -> Self {
        self.0.row = row;
        self
    }

    /// Set title of a [`Table`](crate::Table)
    ///
    /// # None
    ///
    /// When title separator is not preset (i.e., it is `None`), row separator is displayed in place of title separator.
    pub fn title(mut self, title: Option<HorizontalLine>) -> Self {
        self.0.title = title;
        self
    }

    /// Build [`Separator`](crate::format::Separator)
    pub fn build(self) -> Separator {
        self.0
    }
}

/// Struct for configuring a [`Table`](crate::Table)'s format
#[derive(Debug, Default, Copy, Clone)]
pub struct TableFormat {
    pub(crate) border: Border,
    pub(crate) separator: Separator,
    pub(crate) foreground_color: Option<Color>,
    pub(crate) background_color: Option<Color>,
    pub(crate) bold: bool,
}

impl TableFormat {
    /// Creates a new builder for [`TableFormat`](crate::format::TableFormat)
    pub fn builder() -> TableFormatBuilder {
        Default::default()
    }

    pub(crate) fn color_spec(&self) -> ColorSpec {
        let mut color_spec = ColorSpec::new();

        color_spec.set_fg(self.foreground_color);
        color_spec.set_bg(self.background_color);
        color_spec.set_bold(self.bold);

        color_spec
    }
}

/// Builder for [`TableFormat`](crate::format::TableFormat)
#[derive(Debug, Default)]
pub struct TableFormatBuilder(TableFormat);

impl TableFormatBuilder {
    /// Sets border of a [`Table`](crate::Table)
    pub fn border(mut self, border: Border) -> Self {
        self.0.border = border;
        self
    }

    /// Sets separators of a [`Table`](crate::Table)
    pub fn separator(mut self, separator: Separator) -> Self {
        self.0.separator = separator;
        self
    }

    /// Sets foreground colors for borders and separators of a [`Table`](crate::Table)
    pub fn foreground_color(mut self, foreground_color: Option<Color>) -> Self {
        self.0.foreground_color = foreground_color;
        self
    }

    /// Sets background colors for borders and separators of a [`Table`](crate::Table)
    pub fn background_color(mut self, background_color: Option<Color>) -> Self {
        self.0.background_color = background_color;
        self
    }

    /// Sets boldness for borders and separators of a [`Table`](crate::Table)
    #[inline]
    pub fn bold(mut self, bold: bool) -> Self {
        self.0.bold = bold;
        self
    }

    /// Build [`TableFormat`](crate::format::TableFormat)
    pub fn build(self) -> TableFormat {
        self.0
    }
}
