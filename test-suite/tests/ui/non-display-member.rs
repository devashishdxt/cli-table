//! All the members of a struct should implement `Display` (other option is to use `display_fn`)
use cli_table::Table;

#[derive(Table)]
struct Test {
    #[table(title = "a")]
    a: Vec<u8>,
}

fn main() {}
