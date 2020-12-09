use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitStr, Result};

use crate::context::Context;

pub fn title(input: DeriveInput) -> Result<TokenStream> {
    // Create context for generating expressions
    let context = Context::new(&input)?;

    // Used in the quasi-quotation below as `#name`
    let name = context.container.name;

    // Fetch cli_table crate name
    let cli_table = &context.container.crate_name;

    // Split a type's generics into the pieces required for implementing a trait for that type
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Collect all field name literals
    let field_names: Vec<LitStr> = context.fields.into_iter().map(|field| field.name).collect();

    // Build the output, possibly using quasi-quotation
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics Title for #name #ty_generics # where_clause{
            fn title() -> #cli_table ::RowStruct {
                #cli_table ::Row::row(::std::vec![
                    #(#cli_table ::Style::bold(#cli_table ::Cell::cell(#field_names), true),)*
                ])
            }
        }
    })
}
