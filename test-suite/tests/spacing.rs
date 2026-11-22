use cli_table::{
    Cell, ColorChoice, Table, TableStruct,
    format::{Border, Justify, Separator, VerticalLine},
};

fn table() -> TableStruct {
    vec![
        vec!["Tom".cell(), 10.cell().justify(Justify::Right)],
        vec!["Jerry".cell(), 15.cell().justify(Justify::Right)],
    ]
    .table()
    .title(vec!["Name".cell(), "Age".cell()])
    .color_choice(ColorChoice::Never)
}

#[test]
fn default_spacing() {
    let expected = "\
+-------+-----+
| Name  | Age |
+-------+-----+
| Tom   |  10 |
+-------+-----+
| Jerry |  15 |
+-------+-----+";

    assert_eq!(expected, table().display().unwrap().to_string());
}

#[test]
fn zero_spacing_with_borders() {
    let expected = "\
+-----+---+
|Name |Age|
+-----+---+
|Tom  | 10|
+-----+---+
|Jerry| 15|
+-----+---+";

    assert_eq!(expected, table().spacing(0).display().unwrap().to_string());
}

#[test]
fn zero_spacing_without_borders() {
    let expected = "\
Name  Age
Tom    10
Jerry  15";

    let table = table()
        .border(Border::builder().build())
        .separator(
            Separator::builder()
                .column(Some(VerticalLine::new(' ')))
                .build(),
        )
        .spacing(0);

    assert_eq!(expected, table.display().unwrap().to_string());
}
