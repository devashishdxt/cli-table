use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    spanned::Spanned, Data, DeriveInput, Error, Expr, Field as SynField, Fields as SynFields,
    Index, Lit, LitBool, LitStr, Result,
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
    pub title: LitStr,
    pub justify: Option<Expr>,
    pub align: Option<Expr>,
    pub color: Option<Expr>,
    pub bold: Option<LitBool>,
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

        let mut title = None;
        let mut justify = None;
        let mut align = None;
        let mut color = None;
        let mut bold = None;

        let field_attributes = get_attributes(&field.attrs)?;

        for (key, value) in field_attributes {
            if key.is_ident("name") || key.is_ident("title") {
                title = Some(match value {
                    Lit::Str(lit_str) => Ok(lit_str),
                    bad => Err(Error::new_spanned(
                        bad,
                        "Invalid value for #[cli_table(title = \"field_name\")]",
                    )),
                }?);
            } else if key.is_ident("justify") {
                justify = Some(match value {
                    Lit::Str(lit_str) => lit_str.parse::<Expr>(),
                    bad => Err(Error::new_spanned(
                        bad,
                        "Invalid value for #[cli_table(justify = \"value\")]",
                    )),
                }?);
            } else if key.is_ident("align") {
                align = Some(match value {
                    Lit::Str(lit_str) => lit_str.parse::<Expr>(),
                    bad => Err(Error::new_spanned(
                        bad,
                        "Invalid value for #[cli_table(align = \"value\")]",
                    )),
                }?);
            } else if key.is_ident("color") {
                color = Some(match value {
                    Lit::Str(lit_str) => lit_str.parse::<Expr>(),
                    bad => Err(Error::new_spanned(
                        bad,
                        "Invalid value for #[cli_table(color = \"value\")]",
                    )),
                }?);
            } else if key.is_ident("bold") {
                bold = Some(match value {
                    Lit::Bool(lit_bool) => Ok(lit_bool),
                    bad => Err(Error::new_spanned(
                        bad,
                        "Invalid value for #[cli_table(bold)]",
                    )),
                }?);
            }
        }

        let mut field_builder = Self::builder(ident, span);

        if let Some(title) = title {
            field_builder.title(title);
        }

        if let Some(justify) = justify {
            field_builder.justify(justify);
        }

        if let Some(align) = align {
            field_builder.align(align);
        }

        if let Some(color) = color {
            field_builder.color(color);
        }

        if let Some(bold) = bold {
            field_builder.bold(bold);
        }

        Ok(field_builder.build())
    }

    fn builder(ident: TokenStream, span: Span) -> FieldBuilder {
        FieldBuilder::new(ident, span)
    }
}

struct FieldBuilder {
    ident: TokenStream,
    title: Option<LitStr>,
    justify: Option<Expr>,
    align: Option<Expr>,
    color: Option<Expr>,
    bold: Option<LitBool>,
    span: Span,
}

impl FieldBuilder {
    fn new(ident: TokenStream, span: Span) -> Self {
        Self {
            ident,
            title: None,
            justify: None,
            align: None,
            color: None,
            bold: None,
            span,
        }
    }

    fn title(&mut self, title: LitStr) -> &mut Self {
        self.title = Some(title);
        self
    }

    fn justify(&mut self, justify: Expr) -> &mut Self {
        self.justify = Some(justify);
        self
    }

    fn align(&mut self, align: Expr) -> &mut Self {
        self.align = Some(align);
        self
    }

    fn color(&mut self, color: Expr) -> &mut Self {
        self.color = Some(color);
        self
    }

    fn bold(&mut self, bold: LitBool) -> &mut Self {
        self.bold = Some(bold);
        self
    }

    fn build(self) -> Field {
        let ident = self.ident;
        let justify = self.justify;
        let align = self.align;
        let color = self.color;
        let bold = self.bold;
        let span = self.span;

        let title = self
            .title
            .unwrap_or_else(|| LitStr::new(&ident.to_string(), span));

        Field {
            ident,
            title,
            justify,
            align,
            color,
            bold,
            span,
        }
    }
}
