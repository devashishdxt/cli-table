use std::io::Result;

use cli_table::{Align, Cell, CellFormat, Row, Table};

fn main() -> Result<()> {
    let mut format_builder = CellFormat::builder();
    format_builder.align(Align::Center);
    let format = format_builder.build();

    let table = Table::new(
        vec![
            Row::new(vec![
                Cell::new(&format!("Hello\nmy\nname"), format),
                Cell::new("my", format),
            ]),
            Row::new(vec![Cell::new("name", format), Cell::new("is", format)]),
            Row::new(vec![
                Cell::new("Devashish", format),
                Cell::new("Dixit", format),
            ]),
        ],
        Default::default(),
    );

    table.print_std()
}
