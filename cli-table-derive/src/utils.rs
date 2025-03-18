use syn::{spanned::Spanned, Attribute, Error, Lit, LitBool, Path, Result};

pub fn get_attributes(attrs: &[Attribute]) -> Result<Vec<(Path, Lit)>> {
    let mut attributes = Vec::new();

    for attribute in attrs {
        if !attribute.path().is_ident("table") {
            continue;
        }

        if attribute
            .parse_nested_meta(|meta| {
                let path = meta.path.clone();
                let lit = meta
                    .value()
                    .ok()
                    .map(|v| v.parse())
                    .transpose()?
                    .unwrap_or(Lit::from(LitBool {
                        value: true,
                        span: path.span(),
                    }));

                attributes.push((path, lit));

                Ok(())
            })
            .is_err()
        {
            return Err(Error::new_spanned(
                attribute,
                "Attributes should be of type: #[table(key = \"value\", ..)] or #[table(bool)]",
            ));
        }
    }

    Ok(attributes)
}
