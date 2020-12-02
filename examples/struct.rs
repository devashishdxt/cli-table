use std::io::Result;

use cli_table::{format::Justify, print_stdout, Cell, Row, RowStruct, Style, Table, TableStruct};

struct User {
    id: u64,
    first_name: &'static str,
    last_name: &'static str,
}

impl Row for User {
    fn row(self) -> RowStruct {
        vec![
            self.id.cell().justify(Justify::Right),
            self.first_name.cell(),
            self.last_name.cell(),
        ]
        .row()
    }
}

trait Title {
    fn title() -> RowStruct;
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

trait IntoTable {
    fn into_table(self) -> TableStruct;
}

impl<T: Row + Title> IntoTable for Vec<T> {
    fn into_table(self) -> TableStruct {
        let mut data: Vec<RowStruct> = self.into_iter().map(Row::row).collect();
        let title = User::title();

        let mut rows = Vec::with_capacity(data.len() + 1);
        rows.push(title);
        rows.append(&mut data);

        rows.table()
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
    print_stdout(users.into_table())
}
