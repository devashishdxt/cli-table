use std::io::Result;

use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

fn main() -> Result<()> {
    let table = vec![
        vec!["Name".cell().bold(true), "Age (in years)".cell().bold(true)],
        vec!["Tom".cell(), 10.cell().justify(Justify::Right)],
        vec!["Jerry".cell(), 15.cell().justify(Justify::Right)],
        vec!["Scooby Doo".cell(), 20.cell().justify(Justify::Right)],
    ]
    .table()
    .bold(true);

    print_stdout(table)
}
