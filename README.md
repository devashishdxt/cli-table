# CLI Table

![Continuous Integration](https://github.com/devashishdxt/cli-table/workflows/Continuous%20Integration/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/cli-table)](https://crates.io/crates/cli-table)
[![Documentation](https://docs.rs/cli-table/badge.svg)](https://docs.rs/cli-table)
[![License](https://img.shields.io/crates/l/cli-table)](https://github.com/devashishdxt/cli-table/blob/master/LICENSE-MIT)

Rust crate for printing tables on command line.

## Usage

Add `cli-table` in your `Cargo.toms`'s `dependencies` section

```toml
[dependencies]
cli-table = "0.3"
```

Each cell in a `Table` can be formatted using `CellFormat`. `CellFormat` can be easily created like this:

```rust
// Justifies contents of a cell to right
let justify_right = CellFormat::builder().justify(Justify::Right).build();

// Makes contents of a cell bold
let bold = CellFormat::builder().bold(true).build();
```

`Table` can be formatted using `TableFormat`. It is very easy to create a custom table format, but for simplicity, this
crate provides a few predefined `TableFormat`s:

- `BORDER_COLUMN_TITLE`: Format with borders, column separators and row separators (calling `Default::default()` on
  `TableFormat` also returns this format)
- `BORDER_COLUMN_NO_ROW`: Format with borders and column separators (without row separators)
- `BORDER_COLUMN_TITLE`: Format with borders, column separators and title separator (without row separators)
- `BORDER_NO_COLUMN_ROW`: Format with borders and row separators (without column separators)
- `NO_BORDER_COLUMN_ROW`: Format with no borders, column separators and row separators
- `NO_BORDER_COLUMN_TITLE`: Format with no borders, column separators and title separator (without row separators)

To create a table, you can use `Table::new()` like this:

```rust
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
```

To print this table on `stdout`, you can call `table.print_stdout()`.

Below is the output of the table we created just now:

```
+------------+----------------+
| Name       | Age (in years) |  <-- This row will appear in bold
+------------+----------------+
| Tom        |             10 |
+------------+----------------+
| Jerry      |             15 |
+------------+----------------+
| Scooby Doo |             25 |
+------------+----------------+
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as 
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
