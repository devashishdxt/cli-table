use proc_macro2::{Span, TokenStream};
use quote::{quote_spanned, ToTokens};
use syn::{
    spanned::Spanned, Data, DeriveInput, Error, Field as SynField, Fields as SynFields, Index, Lit,
    LitStr, Result,
};

use crate::utils::get_attributes;

pub struct Fields {
    fields: Vec<Field>,
}

impl Fields {
    pub fn new(input: &DeriveInput) -> Result<Self> {
        match input.data {
            Data::Struct(ref data_struct) => Self::from_fields(&data_struct.fields),
            _ => Err(Error::new_spanned(
                input,
                "`cli_table` derive macros can only be used on structs",
            )),
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item = Field> {
        self.fields.into_iter()
    }

    fn from_fields(syn_fields: &SynFields) -> Result<Self> {
        let mut fields = Vec::new();

        for (index, syn_field) in syn_fields.into_iter().enumerate() {
            fields.push(Field::new(syn_field, index)?);
        }

        Ok(Fields { fields })
    }
}

pub struct Field {
    pub ident: TokenStream,
    pub name: LitStr,
    pub justify: TokenStream,
    pub span: Span,
}

impl Field {
    pub fn new(field: &SynField, index: usize) -> Result<Self> {
        let ident = field
            .ident
            .as_ref()
            .map(ToTokens::into_token_stream)
            .unwrap_or_else(|| Index::from(index).into_token_stream());
        let span = field.span();

        let mut name = None;
        let mut justify = None;

        let field_attributes = get_attributes(&field.attrs)?;

        for (key, value) in field_attributes {
            if key.is_ident("name") {
                name = Some(match value {
                    Lit::Str(lit_str) => Ok(lit_str),
                    bad => Err(Error::new_spanned(
                        bad,
                        "Invalid value for #[cli_table(name = \"field_name\")]",
                    )),
                }?);
            } else if key.is_ident("justify") {
                justify = Some(match value {
                    Lit::Str(lit_str) => {
                        let lit_str_value = lit_str.value();

                        match lit_str_value.as_str() {
                            "left" => {
                                Ok(quote_spanned! {span=>
                                    Justify::Left
                                })
                            },
                            "right" => {
                                Ok(quote_spanned! {span=>
                                    Justify::Right
                                })
                            },
                            "center" => {
                                Ok(quote_spanned! {span=>
                                    Justify::Center
                                })
                            },
                            _ => Err(Error::new_spanned(
                                lit_str,
                                "Invalid value for #[cli_table(justify = \"value\")]. Only \"left\", \"right\" and \"center\" is allowed",
                            )),
                        }
                    },
                    bad => Err(Error::new_spanned(
                        bad,
                        "Invalid value for #[cli_table(justify = \"value\")]. Only \"left\", \"right\" and \"center\" is allowed",
                    )),
                }?);
            }
        }

        let mut field_builder = Self::builder(ident, span);

        if let Some(name) = name {
            field_builder.name(name);
        }

        if let Some(justify) = justify {
            field_builder.justify(justify);
        }

        Ok(field_builder.build())
    }

    fn builder(ident: TokenStream, span: Span) -> FieldBuilder {
        FieldBuilder::new(ident, span)
    }
}

struct FieldBuilder {
    ident: TokenStream,
    name: Option<LitStr>,
    justify: Option<TokenStream>,
    span: Span,
}

impl FieldBuilder {
    fn new(ident: TokenStream, span: Span) -> Self {
        Self {
            ident,
            name: None,
            justify: None,
            span,
        }
    }

    fn name(&mut self, name: LitStr) -> &mut Self {
        self.name = Some(name);
        self
    }

    fn justify(&mut self, justify: TokenStream) -> &mut Self {
        self.justify = Some(justify);
        self
    }

    fn build(self) -> Field {
        let ident = self.ident;
        let span = self.span;

        let name = self
            .name
            .unwrap_or_else(|| LitStr::new(&ident.to_string(), span));

        let justify = self.justify.unwrap_or_else(|| {
            quote_spanned! {span=>
                Justify::Left
            }
        });

        Field {
            ident,
            name,
            justify,
            span,
        }
    }
}
