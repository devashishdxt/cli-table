use std::io::Result;

use cli_table::{Cell, Style, Table, format::Justify};

fn main() -> Result<()> {
    let nested_table = vec![vec![20.cell()]].table();

    let table = vec![
        vec!["Tom".cell(), 10.cell().justify(Justify::Right)],
        vec!["Jerry".cell(), 15.cell().justify(Justify::Right)],
        vec!["Scooby Doo".cell(), 20.cell().justify(Justify::Right)],
        vec![
            "Nested".cell(),
            nested_table.display()?.cell().justify(Justify::Right),
        ],
    ]
    .table()
    .title(vec![
        "Name".cell().bold(true),
        "Age (in years)".cell().bold(true),
    ])
    .bold(true);

    println!("{}", table.display()?);

    Ok(())
}
