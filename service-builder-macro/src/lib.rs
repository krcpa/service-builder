use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod builder;
mod field_attributes;

#[proc_macro_attribute]
pub fn builder(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    match builder::expand_builder(input) {
        Ok(expanded) => expanded.into(),
        Err(err) => err.to_compile_error().into(),
    }
}