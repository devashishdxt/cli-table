use crate::{Row, RowStruct, Table, TableStruct};

/// Trait for getting title row of a struct
#[cfg_attr(
    any(docsrs, feature = "doc"),
    doc(cfg(any(feature = "title", feature = "derive")))
)]
pub trait Title {
    /// Returns title row of a struct
    fn title() -> RowStruct;
}

/// Trait for creating a table with titles at the top
#[cfg_attr(
    any(docsrs, feature = "doc"),
    doc(cfg(any(feature = "title", feature = "derive")))
)]
pub trait WithTitle {
    /// Creates a table with title at the top
    fn with_title(self) -> TableStruct;
}

impl<'a, T, R> WithTitle for T
where
    T: IntoIterator<Item = &'a R>,
    R: Title + 'static,
    &'a R: Row,
{
    fn with_title(self) -> TableStruct {
        let mut data: Vec<RowStruct> = self.into_iter().map(|row| row.row()).collect();
        let title = R::title();

        let mut rows = Vec::with_capacity(data.len() + 1);
        rows.push(title);
        rows.append(&mut data);

        rows.table()
    }
}
