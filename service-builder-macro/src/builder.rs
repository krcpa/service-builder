use proc_macro2::{TokenStream, Span};
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, Ident,
};

use crate::field_attributes::{FieldAttributes, DefaultValue};

pub fn expand_builder(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let builder_name = Ident::new(&format!("{}Builder", struct_name), Span::call_site());
    let vis = &input.vis;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields,
            _ => return Err(syn::Error::new(Span::call_site(), "Only named fields are supported")),
        },
        _ => return Err(syn::Error::new(Span::call_site(), "Only structs are supported")),
    };

    let mut field_defs = Vec::new();
    let mut builder_field_defs = Vec::new();
    let mut builder_new_fields = Vec::new();
    let mut builder_methods = Vec::new();
    let mut build_fields = Vec::new();
    let mut build_with_defaults_fields = Vec::new();
    let mut getters = Vec::new();
    let mut setters = Vec::new();

    for field in fields.named.iter() {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let attrs = FieldAttributes::from_field(field, &input.attrs, &field.attrs);

        field_defs.push(quote! {
            #field_name: #field_type
        });

        if attrs.builder {
            builder_field_defs.push(quote! {
                #field_name: std::option::Option<#field_type>
            });

            builder_new_fields.push(quote! {
                #field_name: None
            });

            builder_methods.push(quote! {
                pub fn #field_name(mut self, value: #field_type) -> Self {
                    self.#field_name = Some(value);
                    self
                }
            });

            // For strict build() method
            if attrs.required {
                build_fields.push(quote! {
                    #field_name: self.#field_name.ok_or_else(|| service_builder::error::BuildError::MissingDependency(stringify!(#field_name).to_string()))?
                });
            } else if let Some(default_value) = &attrs.default {
                match default_value {
                    DefaultValue::Default => {
                        build_fields.push(quote! {
                            #field_name: self.#field_name.unwrap_or_default()
                        });
                    }
                    DefaultValue::Expression(expr) => {
                        let expr_tokens: TokenStream = expr.parse().unwrap_or_else(|_| quote! { compile_error!("Invalid default expression") });
                        build_fields.push(quote! {
                            #field_name: self.#field_name.unwrap_or_else(|| #expr_tokens)
                        });
                    }
                }
            } else if attrs.optional {
                // For optional fields without explicit default, use None for Option<T> types
                build_fields.push(quote! {
                    #field_name: self.#field_name.unwrap_or(None)
                });
            } else {
                // No default specified and not marked as optional - this field is still required
                build_fields.push(quote! {
                    #field_name: self.#field_name.ok_or_else(|| service_builder::error::BuildError::MissingDependency(stringify!(#field_name).to_string()))?
                });
            }
            
            // For build_with_defaults() method - always provide a value
            if let Some(default_value) = &attrs.default {
                match default_value {
                    DefaultValue::Default => {
                        build_with_defaults_fields.push(quote! {
                            #field_name: self.#field_name.unwrap_or_default()
                        });
                    }
                    DefaultValue::Expression(expr) => {
                        let expr_tokens: TokenStream = expr.parse().unwrap_or_else(|_| quote! { compile_error!("Invalid default expression") });
                        build_with_defaults_fields.push(quote! {
                            #field_name: self.#field_name.unwrap_or_else(|| #expr_tokens)
                        });
                    }
                }
            } else if attrs.optional {
                build_with_defaults_fields.push(quote! {
                    #field_name: self.#field_name.unwrap_or(None)
                });
            } else {
                // For fields without explicit default, they are still required even in build_with_defaults
                build_with_defaults_fields.push(quote! {
                    #field_name: self.#field_name.ok_or_else(|| service_builder::error::BuildError::MissingDependency(stringify!(#field_name).to_string()))?
                });
            }
        } else {
            build_fields.push(quote! {
                #field_name: Default::default()
            });
            build_with_defaults_fields.push(quote! {
                #field_name: Default::default()
            });
        }

        if attrs.getter {
            let getter_name = Ident::new(&format!("get_{}", field_name), Span::call_site());
            getters.push(quote! {
                pub fn #getter_name(&self) -> &#field_type {
                    &self.#field_name
                }
            });
        }

        if attrs.setter {
            let setter_name = Ident::new(&format!("set_{}", field_name), Span::call_site());
            setters.push(quote! {
                pub fn #setter_name(&mut self, value: #field_type) {
                    self.#field_name = value;
                }
            });
        }
    }

    Ok(quote! {
        #vis struct #struct_name #ty_generics #where_clause {
            #(#field_defs),*
        }

        #vis struct #builder_name #ty_generics #where_clause {
            #(#builder_field_defs),*
        }

        impl #impl_generics #builder_name #ty_generics #where_clause {
            pub fn new() -> Self {
                Self {
                    #(#builder_new_fields),*
                }
            }

            #(#builder_methods)*

            pub fn build(self) -> Result<#struct_name #ty_generics, service_builder::error::BuildError> {
                Ok(#struct_name {
                    #(#build_fields),*
                })
            }
            
            pub fn build_with_defaults(self) -> Result<#struct_name #ty_generics, service_builder::error::BuildError> {
                Ok(#struct_name {
                    #(#build_with_defaults_fields),*
                })
            }
        }

        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn builder() -> #builder_name #ty_generics {
                #builder_name::new()
            }

            #(#getters)*
            #(#setters)*
        }
    })
}