// service-builder/src/lib.rs
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_attribute]
pub fn builder(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let builder_name = format_ident!("{}Builder", name);

    let fields = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => &fields.named,
                _ => panic!("Only named fields are supported")
            }
        },
        _ => panic!("Only structs are supported")
    };

    let field_names: Vec<_> = fields.iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();

    let field_types: Vec<_> = fields.iter()
        .map(|f| &f.ty)
        .collect();

    let builder_fields = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        quote! { #name: Option<#ty> }
    });

    let with_methods = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        quote! {
            pub fn #name(mut self, value: #ty) -> Self {
                self.#name = Some(value);
                self
            }
        }
    });

    let build_checks = field_names.iter().map(|name| {
        let name_str = name.to_string();
        quote! {
            let #name = self.#name.ok_or_else(||
                BuildError::MissingDependency(#name_str.to_string())
            )?;
        }
    });

    let build_fields = field_names.iter().map(|name| {
        quote! { #name: #name }
    });

    let expanded = quote! {
        #input

        pub struct #builder_name {
            #(#builder_fields,)*
        }

        impl #builder_name {
            pub fn new() -> Self {
                Self {
                    #(#field_names: None,)*
                }
            }

            #(#with_methods)*

            pub fn build(self) -> Result<#name, BuildError> {
                #(#build_checks)*

                Ok(#name {
                    #(#build_fields,)*
                })
            }
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name::new()
            }
        }

        #[derive(Debug)]
        pub enum BuildError {
            MissingDependency(String),
        }
    };

    TokenStream::from(expanded)
}