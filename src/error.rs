// service-builder/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("Missing required dependency: {0}")]
    MissingDependency(String),

    #[error("Service initialization failed: {0}")]
    InitializationError(String),

    #[error("Invalid configuration: {0}")]
    ConfigurationError(String),

    #[error("Service build failed: {0}")]
    BuildFailed(String),
}
