use std::io::Result;

use cli_table::{
    Color, Table, WithTitle,
    format::{Align, Justify},
    print_stdout,
};

#[derive(Debug, Table)]
struct User {
    #[table(
        title = "ID",
        justify = "Justify::Right",
        align = "Align::Top",
        color = "Color::Green",
        bold
    )]
    id: u64,
    #[table(title = "First Name")]
    first_name: &'static str,
    #[table(title = "Last Name")]
    last_name: String,
}

fn main() -> Result<()> {
    let users = vec![
        User {
            id: 1,
            first_name: "Scooby",
            last_name: "Doo".to_string(),
        },
        User {
            id: 2,
            first_name: "John",
            last_name: "Cena".to_string(),
        },
    ];

    print_stdout(users.with_title())
}
