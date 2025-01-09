mod mocks;

use std::sync::Arc;
use service_builder::builder;
use service_builder::error::BuildError;
use crate::mocks::{UserRepository, Cache, MockRepository, MockCache};

// Test service with dependencies
#[builder]
#[derive(Debug)]
struct UserService {
    repository: Arc<dyn UserRepository>,
    cache: Arc<dyn Cache>,
}

// Empty test service
#[builder]
#[derive(Debug)]
struct EmptyService {}

#[test]
fn test_service_builder_success() {
    let user_repo = Arc::new(MockRepository);
    let cache = Arc::new(MockCache{});

    let service = UserService::builder()
        .repository(user_repo)
        .cache(cache)
        .build();

    assert!(service.is_ok());
}

#[test]
fn test_missing_dependency_error() {
    let cache = Arc::new(MockCache{});

    let service = UserService::builder()
        .cache(cache)
        .build();

    assert!(matches!(service, Err(BuildError::MissingDependency(_))));
}

#[test]
fn test_empty_service_builder() {
    let service = EmptyService::builder().build();
    assert!(service.is_ok());
}
