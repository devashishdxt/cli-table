use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use crate::context::Context;

pub fn row(input: DeriveInput) -> Result<TokenStream> {
    // Create context for generating expressions
    let context = Context::new(&input)?;

    // Used in the quasi-quotation below as `#name`
    let name = context.container.name;

    // Fetch cli_table crate name
    let cli_table = &context.container.crate_name;

    // Split a type's generics into the pieces required for implementing a trait for that type
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Collect all field expressions
    let fields: Vec<TokenStream> = context
        .fields
        .into_iter()
        .map(|field| {
            let ident = field.ident;
            let justify = field.justify;

            quote!(#cli_table ::Cell::cell(self. #ident).justify(#cli_table ::format:: #justify))
        })
        .collect();

    // Build the output, possibly using quasi-quotation
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics Row for & #name #ty_generics # where_clause{
            fn row(self) -> #cli_table ::RowStruct {
                #cli_table ::Row::row(::std::vec![
                    #(#fields,)*
                ])
            }
        }

        #[automatically_derived]
        impl #impl_generics Row for #name #ty_generics # where_clause{
            fn row(self) -> #cli_table ::RowStruct {
                Row::row(&self)
            }
        }
    })
}
