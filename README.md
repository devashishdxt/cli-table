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
cli-table = "0.4"
```

### Simple usage

```rust
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

let table = vec![
    vec!["Name".cell().bold(true), "Age (in years)".cell().bold(true)],
    vec!["Tom".cell(), 10.cell().justify(Justify::Right)],
    vec!["Jerry".cell(), 15.cell().justify(Justify::Right)],
    vec!["Scooby Doo".cell(), 20.cell().justify(Justify::Right)],
]
.table()
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
use cli_table::{print_stdout, Table, WithTitle};

#[derive(Table)]
struct User {
    #[table(name = "ID", justify = "right")]
    id: u64,
    #[table(name = "First Name")]
    first_name: &'static str,
    #[table(name = "Last Name")]
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
| id | first_name | last_name |  <-- This row will appear in bold
+----+------------+-----------+
| 1  | Scooby     | Doo       |
+----+------------+-----------+
| 2  | John       | Cena      |
+----+------------+-----------+
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
