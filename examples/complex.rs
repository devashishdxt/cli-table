use cli_table::format::{
    Border, CellFormat, HorizontalLine, Justify, Separator, TableFormat, VerticalLine,
};
use cli_table::{Cell, Row, Table};
use std::error::Error;

fn build_top_and_bottom_border_lines() -> (HorizontalLine, HorizontalLine) {
    // Generates a horizontal line as described that we will use for the top of the table
    let top_horizontal_line = HorizontalLine::builder()
        .left_end('┌')
        .right_end('┐')
        .junction('┬')
        .filler('─')
        .build();
    // Generates a horizontal line as described that we will use for the bottom of the table
    let bottom_horizontal_line = HorizontalLine::builder()
        .left_end('└')
        .right_end('┘')
        .junction('┴')
        .filler('─')
        .build();
    (top_horizontal_line, bottom_horizontal_line)
}

fn build_row_separator() -> HorizontalLine {
    HorizontalLine::builder()
        .left_end('├')
        .right_end('┤')
        .junction('┼')
        .filler('─')
        .build()
}

fn build_column_separator() -> VerticalLine {
    VerticalLine::builder().filler('│').build()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Justifies contents of a cell to right
    let justify_right = CellFormat::builder().justify(Justify::Right).build();
    // Makes contents of a cell bold
    let bold = CellFormat::builder().bold(true).build();
    // Build top and bottom border for the table
    let (top, bottom) = build_top_and_bottom_border_lines();
    // Build the row separator for normal rows
    let row_separator = build_row_separator();
    // Build the column separator
    let column_separator = build_column_separator();

    let border = Border::builder()
        .top(top)
        .bottom(bottom)
        .right(column_separator.clone())
        .left(column_separator.clone())
        .build();

    let separator = Separator::builder()
        .row(Some(row_separator))
        .column(Some(column_separator))
        .build();

    let table_format = TableFormat::new(border, separator);

    let table = Table::new(
        vec![
            Row::new(vec![
                Cell::new(&format!("Name"), bold),
                Cell::new("Age (car years)", bold),
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
        table_format,
    )?;
    table.print_stdout()?;
    Ok(())
}
