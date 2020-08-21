use termcolor::Color;

/// A vertical line in a [`Table`](struct.Table.html) (border or column separator)
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
    /// Creates a new instance of [`VerticalLine`](struct.VerticalLine.html)
    pub fn new(filler: char) -> Self {
        Self { filler }
    }
}

/// A horizontal line in a [`Table`](struct.Table.html) (border or row separator)
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
    /// Creates a new instance of [`HorizontalLine`](struct.HorizontalLine.html)
    pub fn new(left_end: char, right_end: char, junction: char, filler: char) -> Self {
        Self {
            left_end,
            right_end,
            junction,
            filler,
        }
    }
}

/// Borders of a [`Table`](struct.Table.html)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Border {
    pub(crate) top: Option<HorizontalLine>,
    pub(crate) bottom: Option<HorizontalLine>,
    pub(crate) left: Option<VerticalLine>,
    pub(crate) right: Option<VerticalLine>,
}

impl Border {
    /// Creates a new builder for [`Border`](struct.Border.html)
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

/// Builder for [`Border`](struct.Border.html)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorderBuilder(Border);

impl BorderBuilder {
    /// Set top border of a [`Table`](struct.Table.html)
    pub fn top(mut self, top: HorizontalLine) -> Self {
        self.0.top = Some(top);
        self
    }

    /// Set bottom border of a [`Table`](struct.Table.html)
    pub fn bottom(mut self, bottom: HorizontalLine) -> Self {
        self.0.bottom = Some(bottom);
        self
    }

    /// Set left border of a [`Table`](struct.Table.html)
    pub fn left(mut self, left: VerticalLine) -> Self {
        self.0.left = Some(left);
        self
    }

    /// Set right border of a [`Table`](struct.Table.html)
    pub fn right(mut self, right: VerticalLine) -> Self {
        self.0.right = Some(right);
        self
    }

    /// Build [`Border`](struct.Border.html)
    pub fn build(self) -> Border {
        self.0
    }
}

/// Inner (column/row) separators of a [`Table`](struct.Table.html)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Separator {
    pub(crate) column: Option<VerticalLine>,
    pub(crate) row: Option<HorizontalLine>,
    pub(crate) title: Option<HorizontalLine>,
}

impl Separator {
    /// Creates a new builder for [`Separator`](struct.Separator.html)
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

/// Builder for [`Separator`](struct.Separator.html)
#[derive(Debug)]
pub struct SeparatorBuilder(Separator);

impl SeparatorBuilder {
    /// Set column separators of a [`Table`](struct.Table.html)
    pub fn column(mut self, column: Option<VerticalLine>) -> Self {
        self.0.column = column;
        self
    }

    /// Set column separators of a [`Table`](struct.Table.html)
    pub fn row(mut self, row: Option<HorizontalLine>) -> Self {
        self.0.row = row;
        self
    }

    /// Set title title of a [`Table`](struct.Table.html)
    ///
    /// # None
    ///
    /// When title separator is not preset (i.e., it is `None`), row separator is displayed in place of title separator.
    pub fn title(mut self, title: Option<HorizontalLine>) -> Self {
        self.0.title = title;
        self
    }

    /// Build [`Separator`](struct.Separator.html)
    pub fn build(self) -> Separator {
        self.0
    }
}

/// Struct for configuring a [`Table`](struct.Table.html)'s format
#[derive(Debug, Default)]
pub struct TableFormat {
    pub(crate) foreground: Option<Color>,
    pub(crate) border: Border,
    pub(crate) separator: Separator,
}

impl TableFormat {
    /// Creates a new instance of [`TableFormat`](struct.TableFormat.html)
    pub fn new(border: Border, separator: Separator) -> Self {
        Self {
            border,
            separator,
            foreground: None
        }
    }

    /// Set the foreground color of the table borders & separators.
    pub fn foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }
}
