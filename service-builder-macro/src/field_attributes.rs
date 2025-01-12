use syn::{Attribute, Field, parse::Parse, Token, punctuated::Punctuated, parse::ParseStream};
use quote::ToTokens;

#[derive(Debug, Default)]
pub struct FieldAttributes {
    pub getter: bool,
    pub setter: bool,
    pub builder: bool,
    pub required: bool,
}

#[derive(Debug)]
struct FieldConfig {
    name: String,
    getter: bool,
    setter: bool,
}

#[derive(Debug)]
struct FieldConfigs(Punctuated<FieldConfig, Token![,]>);

impl Parse for FieldConfigs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(FieldConfigs(Punctuated::parse_terminated(input)?))
    }
}

impl Parse for FieldConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?.to_string();
        let content;
        syn::parenthesized!(content in input);
        let mut getter = false;
        let mut setter = false;

        let nested_meta = content.parse_terminated(syn::Meta::parse, Token![,])?;
        for meta in nested_meta {
            match meta {
                syn::Meta::Path(path) => {
                    let ident = path.get_ident().unwrap().to_string();
                    if ident == "getter" {
                        getter = true;
                    } else if ident == "setter" {
                        setter = true;
                    }
                },
                _ => {}
            }
        }

        Ok(FieldConfig {
            name,
            getter,
            setter,
        })
    }
}

impl FieldAttributes {
    pub fn from_field(field: &Field, struct_attrs: &[Attribute], field_attrs: &[Attribute]) -> Self {
        let mut attrs = FieldAttributes {
            builder: true, // Default to true for backward compatibility
            required: true, // Default to true for backward compatibility
            ..Default::default()
        };
        let field_name = field.ident.as_ref().unwrap().to_string();

        // Process field-level attributes first
        for attr in field_attrs {
            if attr.path().is_ident("builder") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("getter") {
                        attrs.getter = true;
                    } else if meta.path.is_ident("setter") {
                        attrs.setter = true;
                    } else if meta.path.is_ident("skip") {
                        attrs.builder = false;
                    } else if meta.path.is_ident("optional") {
                        attrs.required = false;
                    }
                    Ok(())
                });
            }
        }

        // Process struct-level attributes second (they can override field-level attributes)
        for attr in struct_attrs {
            if !attr.path().is_ident("builder") {
                continue;
            }

            if let Ok(meta) = attr.parse_args_with(|input: syn::parse::ParseStream| {
                input.parse_terminated(FieldConfig::parse, Token![,])
            }) {
                for config in meta {
                    if config.name == field_name {
                        attrs.getter |= config.getter;
                        attrs.setter |= config.setter;
                    }
                }
            }
        }

        attrs
    }
}