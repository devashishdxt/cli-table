/// A vertical line in a [`Table`](struct.Table.html) (border or column separator)
#[derive(Debug, Clone)]
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

    /// Creates a new builder for [`VerticalLine`](struct.VerticalLine.html)
    pub fn builder() -> VerticalLineBuilder {
        VerticalLineBuilder(VerticalLine::default())
    }
}

/// Builder for [`HorizontalLine`](struct.Border.html)
#[derive(Debug)]
pub struct VerticalLineBuilder(VerticalLine);

impl VerticalLineBuilder {
    pub fn build(self) -> VerticalLine {
        self.0
    }

    pub fn filler(mut self, value: char) -> Self {
        self.0.filler = value;
        self
    }
}

/// A horizontal line in a [`Table`](struct.Table.html) (border or row separator)
#[derive(Debug)]
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

    /// Creates a new builder for [`HorizontalLine`](struct.HorizontalLine.html)
    pub fn builder() -> HorizontalLineBuilder {
        HorizontalLineBuilder(HorizontalLine::default())
    }
}

/// Builder for [`HorizontalLine`](struct.Border.html)
#[derive(Debug)]
pub struct HorizontalLineBuilder(HorizontalLine);

impl HorizontalLineBuilder {
    pub fn build(self) -> HorizontalLine {
        self.0
    }

    pub fn left_end(mut self, value: char) -> Self {
        self.0.left_end = value;
        self
    }
    pub fn right_end(mut self, value: char) -> Self {
        self.0.right_end = value;
        self
    }
    pub fn junction(mut self, value: char) -> Self {
        self.0.junction = value;
        self
    }
    pub fn filler(mut self, value: char) -> Self {
        self.0.filler = value;
        self
    }
}

/// Borders of a [`Table`](struct.Table.html)
#[derive(Debug)]
pub struct Border {
    pub(crate) top: Option<HorizontalLine>,
    pub(crate) bottom: Option<HorizontalLine>,
    pub(crate) left: Option<VerticalLine>,
    pub(crate) right: Option<VerticalLine>,
}

impl Border {
    /// Creates a new builder for [`Border`](struct.Border.html)
    pub fn builder() -> BorderBuilder {
        BorderBuilder(Border::default())
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
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Separator {
    pub(crate) column: Option<VerticalLine>,
    pub(crate) row: Option<HorizontalLine>,
    pub(crate) title: Option<HorizontalLine>,
}

impl Separator {
    /// Creates a new builder for [`Separator`](struct.Separator.html)
    pub fn builder() -> SeparatorBuilder {
        SeparatorBuilder(Default::default())
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
    pub(crate) border: Border,
    pub(crate) separator: Separator,
}

impl TableFormat {
    /// Creates a new instance of [`TableFormat`](struct.TableFormat.html)
    pub fn new(border: Border, separator: Separator) -> Self {
        Self { border, separator }
    }
}
