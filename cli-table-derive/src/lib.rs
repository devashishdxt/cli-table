#![forbid(unsafe_code)]
#![cfg_attr(not(any(docsrs, feature = "doc")), forbid(unstable_features))]
#![deny(missing_docs)]
#![cfg_attr(any(docsrs, feature = "doc"), feature(doc_cfg))]
//! Derive macros for `cli-table` crate.
//!
//! For more details, see [`cli-table`](https://docs.rs/cli-table).
mod context;
mod table;
mod utils;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Table, attributes(table))]
/// Derive macro to implementing `cli_table` traits
pub fn table(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Prepare and return the output
    table::table(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
