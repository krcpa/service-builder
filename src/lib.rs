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
//! - Comprehensive error handling
//!
//! # Example
//!
//! ```rust
//! use service_builder::builder;
//! use service_builder::error::BuildError;
//! use std::sync::Arc;
//!
//! #[builder]
//! struct UserService {
//!     repository: Arc<dyn UserRepository>,
//!     cache: Arc<dyn Cache>,
//! }
//!
//! let service = UserService::builder()
//!     .repository(user_repo)
//!     .cache(cache)
//!     .build()?;
//! ```
// error.rs
//! Error types for the service builder.
//!
//! This module provides a comprehensive error handling system for service initialization
//! and building processes.


pub mod error;

pub use service_builder_macro::builder;