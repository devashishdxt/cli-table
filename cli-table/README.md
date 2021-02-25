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
- `skip`: Used to skip a field from table. Usage: `#[table(skip)]`

For more information on configurations available on derive macro, go to `cli-table/examples/struct.rs`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.