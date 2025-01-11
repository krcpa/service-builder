mod mocks;

use std::sync::Arc;
use service_builder::builder;
use crate::mocks::{UserRepository, Cache, MockRepository, MockCache, AsUserRepository};

// Test service with dependencies
#[builder]
pub struct UserService {
    repository: Arc<MockRepository>,
    cache: Arc<MockCache>,
}

// Test empty service
#[builder]
pub struct EmptyService {}

#[test]
fn test_repository_builder() {
    let repository = Arc::new(MockRepository);
    assert!(repository.as_user_repository().is_some());
}

#[test]
fn test_cache_builder() {
    let cache = Arc::new(MockCache::builder()
        .value("test".to_string())
        .build()
        .unwrap());
    assert!(!Arc::as_ptr(&cache).is_null());
}

#[test]
fn test_repository_initialization() {
    let repository = Arc::new(MockRepository);
    let result = repository.initialize();
    assert!(result.is_ok());
}

#[test]
fn test_cache_initialization() {
    let cache = Arc::new(MockCache::builder()
        .value("test".to_string())
        .build()
        .unwrap());
    assert!(!Arc::as_ptr(&cache).is_null());
}

#[test]
fn test_service_builder_success() {
    let user_repo = Arc::new(MockRepository);
    let cache = Arc::new(MockCache::builder()
        .value("test".to_string())
        .build()
        .unwrap());

    let service = UserService::builder()
        .repository(user_repo)
        .cache(cache)
        .build();
    assert!(service.is_ok());
}

#[test]
fn test_missing_dependency_error() {
    let cache = Arc::new(MockCache::builder()
        .value("test".to_string())
        .build()
        .unwrap());

    let service = UserService::builder()
        .cache(cache)
        .build();
    assert!(service.is_err());
}

#[test]
fn test_empty_service_builder() {
    let service = EmptyService::builder().build();
    assert!(service.is_ok());
}
