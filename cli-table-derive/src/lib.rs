mod context;
mod row;
mod title;
mod utils;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Title, attributes(cli_table))]
pub fn title(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Prepare and return the output
    title::title(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(Row, attributes(cli_table))]
pub fn row(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Prepare and return the output
    row::row(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
