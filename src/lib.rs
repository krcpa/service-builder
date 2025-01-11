//! Service Builder is a lightweight, type-safe service construction library that leverages
//! the builder pattern for dependency injection in Rust.
//!
//! # Overview
//!
//! This crate provides a procedural macro for automatically implementing the builder pattern
//! for service initialization. It focuses on compile-time safety, zero-cost abstractions,
//! and idiomatic Rust patterns.
//!
//! # Features
//!
//! - Type-safe dependency injection at compile time
//! - Zero runtime overhead
//! - Automatic builder implementation via proc-macros
//! - Field-level getter and setter generation
//! - Comprehensive error handling
//!
//! # Example
//!
//! ```ignore
//! use service_builder::builder;
//! use service_builder::error::BuildError;
//! use std::sync::Arc;
//!
//! #[builder]
//! struct UserService {
//!     #[builder(getter)]  // Generates a get_repository method
//!     repository: Arc<dyn UserRepository>,
//!     #[builder(setter)]  // Generates a set_cache method
//!     cache: Arc<dyn Cache>,
//!     #[builder(getter, setter)]  // Generates both getter and setter
//!     config: ServiceConfig,
//! }
//!
//! let service = UserService::builder()
//!     .repository(user_repo)
//!     .cache(cache)
//!     .config(config)
//!     .build()?;
//!
//! // Access fields using generated getters
//! let repo = service.get_repository();
//!
//! // Modify fields using generated setters
//! service.set_cache(new_cache);
//! ```
//!
//! # Field Attributes
//!
//! The `builder` attribute supports the following field-level options:
//!
//! - `#[builder(getter)]`: Generates a getter method for the field
//! - `#[builder(setter)]`: Generates a setter method for the field
//! - Both can be combined: `#[builder(getter, setter)]`
//!
//! Generated methods follow these naming conventions:
//! - Getters: `get_field_name() -> &FieldType`
//! - Setters: `set_field_name(value: FieldType)`

pub mod error;
pub mod prelude;

pub use service_builder_macro::builder;