use cli_table::{
    format::{CellFormat, Justify},
    Cell, Row, Table,
};
use std::error::Error;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let justify_right = CellFormat::builder().justify(Justify::Right).build();
    let bold = CellFormat::builder().bold(true).build();

    let table = Table::new(
        vec![
            Row::new(vec![
                Cell::new(&format!("Name"), bold),
                Cell::new("Age (in years)", bold),
            ]),
            Row::new(vec![
                Cell::new("Tom", Default::default()),
                Cell::new("10", justify_right),
            ]),
            Row::new(vec![
                Cell::new("Jerry", Default::default()),
                Cell::new("15", justify_right),
            ]),
            Row::new(vec![
                Cell::new("Scooby Doo", Default::default()),
                Cell::new("25", justify_right),
            ]),
        ],
        Default::default(),
    )?;

    table.print_stdout()?;

    Ok(())
}
