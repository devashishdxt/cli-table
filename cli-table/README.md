# cli-table

[![Continuous Integration](https://github.com/devashishdxt/cli-table/workflows/Continuous%20Integration/badge.svg)](https://github.com/devashishdxt/cli-table/actions?query=workflow%3A%22Continuous+Integration%22)
[![Crates.io](https://img.shields.io/crates/v/cli-table)](https://crates.io/crates/cli-table)
[![Documentation](https://docs.rs/cli-table/badge.svg)](https://docs.rs/cli-table)
[![License](https://img.shields.io/crates/l/cli-table)](https://github.com/devashishdxt/cli-table/blob/master/LICENSE-MIT)

Rust crate for printing tables on command line.

## Usage

Add `cli-table` in your `Cargo.toms`'s `dependencies` section

```toml
[dependencies]
cli-table = "0.4"
```

### Simple usage

```rust
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

let table = vec![
    vec!["Tom".cell(), 10.cell().justify(Justify::Right)],
    vec!["Jerry".cell(), 15.cell().justify(Justify::Right)],
    vec!["Scooby Doo".cell(), 20.cell().justify(Justify::Right)],
]
.table()
.title(vec![
    "Name".cell().bold(true),
    "Age (in years)".cell().bold(true),
])
.bold(true);

assert!(print_stdout(table).is_ok());
```

Below is the output of the table we created just now:

```markdown
+------------+----------------+
| Name       | Age (in years) |  <-- This row and all the borders/separators
+------------+----------------+      will appear in bold
| Tom        |             10 |
+------------+----------------+
| Jerry      |             15 |
+------------+----------------+
| Scooby Doo |             25 |
+------------+----------------+
```

### `Display` trait implementation

To get a `Display` trait implementation of `TableStruct`, use `display()` function on the struct to get an instance
of `TableDisplay` which implements `Display` trait.

```rust
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

let table = vec![
    vec!["Tom".cell(), 10.cell().justify(Justify::Right)],
    vec!["Jerry".cell(), 15.cell().justify(Justify::Right)],
    vec!["Scooby Doo".cell(), 20.cell().justify(Justify::Right)],
]
.table()
.title(vec![
    "Name".cell().bold(true),
    "Age (in years)".cell().bold(true),
])
.bold(true);

let table_display = table.display().unwrap();

println!("{}", table_display);
```

Below is the output of the table we created just now:

```markdown
+------------+----------------+
| Name       | Age (in years) |  <-- This row and all the borders/separators
+------------+----------------+      will appear in bold
| Tom        |             10 |
+------------+----------------+
| Jerry      |             15 |
+------------+----------------+
| Scooby Doo |             25 |
+------------+----------------+
```

### Derive macro

`#[derive(Table)]` can also be used to print a `Vec` or slice of `struct`s as table.

```rust
use cli_table::{format::Justify, print_stdout, Table, WithTitle};

#[derive(Table)]
struct User {
    #[table(title = "ID", justify = "Justify::Right")]
    id: u64,
    #[table(title = "First Name")]
    first_name: &'static str,
    #[table(title = "Last Name")]
    last_name: &'static str,
}

let users = vec![
    User {
        id: 1,
        first_name: "Scooby",
        last_name: "Doo",
    },
    User {
        id: 2,
        first_name: "John",
        last_name: "Cena",
    },
];

assert!(print_stdout(users.with_title()).is_ok());
```

Below is the output of the table we created using derive macro:

```markdown
+----+------------+-----------+
| ID | First Name | Last Name |  <-- This row will appear in bold
+----+------------+-----------+
|  1 | Scooby     | Doo       |
+----+------------+-----------+
|  2 | John       | Cena      |
+----+------------+-----------+
```

#### Field attributes

- `title` | `name`: Used to specify title of a column. Usage: `#[table(title = "Title")]`
- `justify`: Used to horizontally justify the contents of a column. Usage: `#[table(justify = "Justify::Right")]`
- `align`: Used to vertically align the contents of a column. Usage: `#[table(align = "Align::Top")]`
- `color`: Used to specify color of contents of a column. Usage: `#[table(color = "Color::Red")]`
- `bold`: Used to specify boldness of contents of a column. Usage: `#[table(bold)]`
- `order`: Used to order columns in a table while printing. Usage: `#[table(order = <usize>)]`. Here, columns will
  be sorted based on their order. For e.g., column with `order = 0` will be displayed on the left followed by
  column with `order = 1` and so on.
- `display_fn`: Used to print types which do not implement `Display` trait. Usage `#[table(display_fn = "<func_name>")]`.
  Signature of provided function should be `fn <func_name>(value: &<type>) -> impl Display`.
- `customize_fn`: Used to customize style of a cell. Usage `#[table(customize_fn = "<func_name>")]`. Signature of
  provided function should be `fn <func_name>(cell: CellStruct, value: &<type>) -> CellStruct`. This attribute can
  be used when you want to change the formatting/style of a cell based on its contents. Note that this will
  overwrite all the style settings done by other attributes.
- `skip`: Used to skip a field from table. Usage: `#[table(skip)]`

For more information on configurations available on derive macro, go to `cli-table/examples/struct.rs`.

### CSV

This crate also integrates with [`csv`](https://crates.io/crates/csv) crate. On enabling `"csv"` feature, you can
use `TryFrom<&mut Reader> for TableStruct` trait implementation to convert `csv::Reader` to `TableStruct`.

For more information on handling CSV values, go to `cli-table/examples/csv.rs`.

## Styling

Style of a table/cell can be modified by calling functions of [`Style`] trait. It is implementated by both
[`TableStruct`] and [`CellStruct`].

For individually formatting each cell of a table, `justify`, `align` and `padding` functions can be used from
`CellStruct`.

In addition to this, borders and separators of a table can be customized by calling `border` and `separator`
functions in `TableStruct`. For example, to create a borderless table:

```rust
use cli_table::{Cell, Table, TableStruct, format::{Justify, Border}, print_stdout};

fn get_table() -> TableStruct {
    vec![
        vec!["Tom".cell(), 10.cell().justify(Justify::Right)],
        vec!["Jerry".cell(), 15.cell().justify(Justify::Right)],
        vec!["Scooby Doo".cell(), 20.cell().justify(Justify::Right)],
    ]
    .table()
}

let table = get_table().border(Border::builder().build()); // Attaches an empty border to the table
assert!(print_stdout(table).is_ok());
```

## Features

- `derive`: Enables derive macro for creating tables using structs. **Enabled** by default.
- `csv`: Enables support for printing tables using [`csv`](https://crates.io/crates/csv). **Enabled** by default.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
