//! `Table` derive macro is not allowed on enums
use cli_table::Table;

#[derive(Table)]
enum Test {
    Variant1,
    Variant2,
}

fn main () {}
