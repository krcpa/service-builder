use proc_macro::TokenStream;

pub(crate) fn impl_setter(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // The actual implementation is handled in the builder macro
    // This just serves as a marker attribute
    item
}