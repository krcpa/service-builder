//! Procedural macros for the service-builder crate.
//!
//! This crate provides the implementation of the `builder` attribute macro,
//! which generates builder pattern code along with optional getters and setters.

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod builder;
mod field_attributes;

/// Implements the builder pattern for a struct, with optional getter and setter methods.
///
/// # Field Attributes
///
/// - `#[builder(getter)]`: Generates a getter method for the field
/// - `#[builder(setter)]`: Generates a setter method for the field
/// - `#[builder(getter, setter)]`: Generates both getter and setter methods
///
/// # Example
///
/// ```rust
/// use service_builder::builder;
///
/// #[builder]
/// struct Service {
///     #[builder(getter)]
///     name: String,
///     #[builder(setter)]
///     count: i32,
///     #[builder(getter, setter)]
///     enabled: bool,
/// }
///
/// let mut service = Service::builder()
///     .name("test".to_string())
///     .count(42)
///     .enabled(true)
///     .build()
///     .unwrap();
///
/// // Use generated getter
/// assert_eq!(service.get_name(), &"test".to_string());
///
/// // Use generated setter
/// service.set_count(100);
/// service.set_enabled(false);
/// ```
#[proc_macro_attribute]
pub fn builder(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    match builder::expand_builder(input) {
        Ok(expanded) => expanded.into(),
        Err(err) => err.to_compile_error().into(),
    }
}