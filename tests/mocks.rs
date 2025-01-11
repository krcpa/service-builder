use service_builder_macro::builder;

// Define mock traits for testing
pub trait UserRepository: std::fmt::Debug {
    fn initialize(&self) -> Result<(), String> {
        Ok(()) // By default, initialization succeeds
    }
}
pub trait Cache: std::fmt::Debug {}

// Mock implementations
#[derive(Debug)]
pub struct MockRepository;
impl UserRepository for MockRepository {}

#[derive(Debug)]
#[builder]
pub struct MockCache {
    pub value: String,
}
impl Cache for MockCache {}

// Helper trait for checking if a type implements UserRepository
pub trait AsUserRepository {
    fn as_user_repository(&self) -> Option<&dyn UserRepository>;
}

// Implement for Arc<dyn UserRepository>
impl AsUserRepository for std::sync::Arc<dyn UserRepository> {
    fn as_user_repository(&self) -> Option<&dyn UserRepository> {
        Some(self.as_ref())
    }
}

// Implement for Arc<T> where T: UserRepository
impl<T: UserRepository> AsUserRepository for std::sync::Arc<T> {
    fn as_user_repository(&self) -> Option<&dyn UserRepository> {
        Some(self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_as_user_repository() {
        let repo = Arc::new(MockRepository);
        assert!(repo.as_user_repository().is_some());
    }
}
