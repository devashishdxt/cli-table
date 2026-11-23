use cli_table::{
    Cell, Color, ColorChoice, Style, Table,
    format::{Border, Justify, Padding, Separator},
};

#[test]
fn default_cell_padding() {
    let table = vec![
        vec!["Tom".cell(), 10.cell().justify(Justify::Right)],
        vec!["Jerry".cell(), 15.cell().justify(Justify::Right)],
    ]
    .table()
    .title(vec!["Name".cell(), "Age".cell()])
    .color_choice(ColorChoice::Never);

    let expected = "\
+-------+-----+
| Name  | Age |
+-------+-----+
| Tom   |  10 |
+-------+-----+
| Jerry |  15 |
+-------+-----+";

    assert_eq!(expected, table.display().unwrap().to_string());
}

#[test]
fn zero_padding_replaces_defaults() {
    let table = vec![vec![
        "Name".cell().padding(Padding::default()),
        "Age".cell().padding(Padding::default()),
    ]]
    .table()
    .color_choice(ColorChoice::Never);

    assert_eq!(
        "+----+---+\n|Name|Age|\n+----+---+",
        table.display().unwrap().to_string(),
    );
}

#[test]
fn custom_padding_sets_column_width_and_row_height() {
    let padding = Padding::builder().left(4).top(1).bottom(1).build();
    let table = vec![vec![
        "Tom".cell().padding(padding),
        10.cell().justify(Justify::Right),
    ]]
    .table()
    .title(vec!["Name".cell(), "Age".cell()])
    .color_choice(ColorChoice::Never);

    let expected = "\
+-------+-----+
| Name  | Age |
+-------+-----+
|       |  10 |
|    Tom|     |
|       |     |
+-------+-----+";

    assert_eq!(expected, table.display().unwrap().to_string());
}

#[test]
fn cell_padding_uses_cell_background_color() {
    let rendered = vec![vec!["Tom".cell().background_color(Some(Color::Blue))]]
        .table()
        .border(Border::builder().build())
        .separator(Separator::builder().build())
        .color_choice(ColorChoice::AlwaysAnsi)
        .display()
        .unwrap()
        .to_string();

    assert!(rendered.contains("\x1b[44m Tom \x1b[0m"), "{rendered:?}");
}
