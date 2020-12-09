use syn::{Attribute, Error, Lit, Meta, NestedMeta, Path, Result};

pub fn get_attributes(attrs: &[Attribute]) -> Result<Vec<(Path, Lit)>> {
    let mut attributes = Vec::new();

    for attribute in attrs {
        if !attribute.path.is_ident("cli_table") {
            continue;
        }

        let meta = attribute.parse_meta()?;

        let meta_list = match meta {
            Meta::List(meta_list) => Ok(meta_list),
            bad => Err(Error::new_spanned(
                bad,
                "Attributes should be of type: #[cli_table(key = \"value\", ..)]",
            )),
        }?;

        for nested_meta in meta_list.nested.into_iter() {
            let meta = match nested_meta {
                NestedMeta::Meta(meta) => Ok(meta),
                bad => Err(Error::new_spanned(
                    bad,
                    "Attributes should be of type: #[cli_table(key = \"value\", ..)]",
                )),
            }?;

            let meta_name_value = match meta {
                Meta::NameValue(meta_name_value) => Ok(meta_name_value),
                bad => Err(Error::new_spanned(
                    bad,
                    "Attributes should be of type: #[cli_table(key = \"value\", ..)]",
                )),
            }?;

            attributes.push((meta_name_value.path, meta_name_value.lit));
        }
    }

    Ok(attributes)
}
