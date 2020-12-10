use std::io::Result;

use cli_table::{print_stdout, Table, WithTitle};

#[derive(Debug, Table)]
struct User {
    #[table(name = "ID", justify = "right")]
    id: u64,
    #[table(name = "First Name")]
    first_name: &'static str,
    #[table(name = "Last Name")]
    last_name: &'static str,
}

fn main() -> Result<()> {
    let users = vec![
        User {
            id: 1,
            first_name: "Scooby",
            last_name: "Doo",
        },
        User {
            id: 2,
            first_name: "John",
            last_name: "Cena",
        },
    ];

    print_stdout(users.with_title())
}
