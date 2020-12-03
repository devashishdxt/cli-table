use std::io::Result;

use cli_table::{format::Justify, print_stdout, Cell, Row, RowStruct, Style, Title, WithTitle};

#[derive(Debug)]
struct User {
    id: u64,
    first_name: &'static str,
    last_name: &'static str,
}

impl Row for &User {
    fn row(self) -> RowStruct {
        vec![
            self.id.cell().justify(Justify::Right),
            self.first_name.cell(),
            self.last_name.cell(),
        ]
        .row()
    }
}

impl Row for User {
    fn row(self) -> RowStruct {
        (&self).row()
    }
}

impl Title for User {
    fn title() -> RowStruct {
        vec![
            "id".cell().bold(true),
            "first_name".cell().bold(true),
            "last_name".cell().bold(true),
        ]
        .row()
    }
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

    print_stdout(users.with_title())?;
    println!("{:?}", users);

    Ok(())
}
