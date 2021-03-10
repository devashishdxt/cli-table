//! `title` only accepts `&str` values
use cli_table::Table;

#[derive(Table)]
struct Test {
    #[table(title = 1)]
    a: u8,
}

fn main() {}
