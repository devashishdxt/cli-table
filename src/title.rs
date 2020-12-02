use crate::RowStruct;

/// Trait for getting title row of a struct
pub trait Title {
    /// Returns title row of a struct
    fn title() -> RowStruct;
}
