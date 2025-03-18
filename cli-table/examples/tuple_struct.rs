use std::io::Result;

use cli_table::{
    Table, WithTitle,
    format::{HorizontalLine, Justify::Right, Separator},
    print_stdout,
};

#[derive(Debug, Table)]
struct User(
    #[table(title = "ID", justify = "Right")] u64,
    #[table(title = "First Name")] &'static str,
    #[table(title = "Last Name")] &'static str,
);

fn main() -> Result<()> {
    let users = vec![
        User(1, "Scooby", "Doo"),
        User(2, "John", "Cena"),
        User(3, "Sherlock", "Holmes"),
    ];
    let table = users.with_title().separator(
        Separator::builder()
            .title(Some(HorizontalLine::new('+', '+', '+', '=')))
            .row(Some(Default::default()))
            .column(Some(Default::default()))
            .build(),
    );
    print_stdout(table)
}
