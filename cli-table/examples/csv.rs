use std::{convert::TryFrom, io::Result};

use cli_table::{TableStruct, print_stdout};
use csv::ReaderBuilder;

fn main() -> Result<()> {
    let data = "\
city,country,pop
Boston,United States,4628910
Concord,United States,42695
";

    let mut reader = ReaderBuilder::new().from_reader(data.as_bytes());
    let table = TableStruct::try_from(&mut reader).unwrap();

    print_stdout(table)
}
