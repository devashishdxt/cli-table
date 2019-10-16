# CLI Table

Rust crate for printing tables on command line.

## Usage

Add `cli-table` in your `Cargo.toms`'s `dependencies` section

```toml
[dependencies]
cli-table = "0.1"
```

Each cell in a `Table` can be formatted using `CellFormat`. `CellFormat` can be easily created like this:

```rust
// Justifies contents of a cell to right
let justify_right = CellFormat::builder().justify(Justify::Right).build();

// Makes contents of a cell bold
let bold = CellFormat::builder().bold(true).build();
```

To create a table, you can use `Table::new()` like this:

```rust
let table = Table::new(vec![
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
]);
```

To print this table on `stdout`, you can call `table.print_std()`.

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
- Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as 
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
