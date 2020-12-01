use cli_table::{
    format::{CellFormat, Color, Justify, Padding, TableFormat},
    Cell, Row, Table,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let justify_right = CellFormat::builder()
        .justify(Justify::Right)
        .padding(Padding::builder().right(2).build())
        .build();
    let bold = CellFormat::builder().bold(true).build();

    let mut table = Table::new(
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
        TableFormat::builder()
            .foreground_color(Some(Color::Red))
            .build(),
    )?;

    table.print_stdout()?;

    Ok(())
}
