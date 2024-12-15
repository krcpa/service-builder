# service-builder (WIP)

> Not building yet

[![Crates.io](https://img.shields.io/crates/v/service-builder.svg)](https://crates.io/crates/service-builder)
[![Documentation](https://docs.rs/service-builder/badge.svg)](https://docs.rs/service-builder)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightweight, type-safe service construction library for Rust that leverages the builder pattern to provide a more idiomatic alternative to traditional dependency injection.

## Why Builder Pattern in Rust?

### 1. Ownership and Borrowing
Traditional dependency injection frameworks often struggle with Rust's ownership system. The builder pattern works naturally with Rust's ownership rules:

```rust
// ❌ Traditional DI approach - fights with ownership
container.register::<UserService>(UserService::new);
let service = container.resolve::<UserService>().unwrap(); // Runtime checks

// ✅ Builder pattern - works with ownership
let service = UserService::builder()
    .repository(repo)
    .cache(cache)
    .build()?; // Compile-time checks
```

### 2. Compile-Time Guarantees
Rust's type system can catch dependency issues at compile time with the builder pattern:

```rust
#[builder]
struct UserService {
    repository: Arc<dyn Repository>,
    cache: Arc<dyn Cache>,
}

// Won't compile if you forget a dependency
let service = UserService::builder()
    .repository(repo)
    // Forgot .cache()
    .build(); // Compile error!
```

### 3. Clear Dependency Flow
Dependencies are explicit and visible in the code:

```rust
let auth_service = AuthService::builder()
    .user_repository(user_repo)
    .token_service(token_service)
    .build()?;

let post_service = PostService::builder()
    .post_repository(post_repo)
    .auth_service(auth_service) // Clear dependency chain
    .build()?;
```

## Quick Start

Add this to your `Cargo.toml`:
```toml
[dependencies]
service-builder = "0.1.0"
```

Basic usage:
```rust
use service_builder::prelude::*;
use std::sync::Arc;

#[builder]
struct UserService {
    repository: Arc<dyn UserRepository>,
    cache: Arc<dyn Cache>,
}

#[builder]
struct AppServices {
    user_service: Arc<UserService>,
    post_service: Arc<PostService>,
}

// In your main.rs or setup code
let user_service = UserService::builder()
    .repository(user_repo)
    .cache(cache)
    .build()?;

let app_services = AppServices::builder()
    .user_service(Arc::new(user_service))
    .post_service(Arc::new(post_service))
    .build()?;
```

## Builder Pattern vs Traditional DI

### Memory Safety and Ownership
```rust
// ❌ DI Container - Potential runtime panics
let service = container.resolve::<Service>().unwrap();

// ✅ Builder Pattern - Ownership is clear and enforced
let service = Service::builder()
    .dependency(dep)
    .build()?;
```

### Type Safety
```rust
// ❌ DI Container - Runtime type checks
container.register::<dyn Repository>(Box::new(MyRepo));

// ✅ Builder Pattern - Compile-time type checks
#[builder]
struct Service {
    repo: Arc<dyn Repository>
}
```

### Testing
```rust
// ✅ Easy mock injection with builder
#[test]
fn test_service() {
    let mock_repo = Arc::new(MockRepository::new());
    let service = Service::builder()
        .repository(mock_repo)
        .build()
        .unwrap();
    
    // Test your service
}
```

## Advanced Features

### Error Handling
```rust
#[derive(Debug, Error)]
pub enum BuildError {
    #[error("Missing dependency: {0}")]
    MissingDependency(String),
    // ... other error types
}

// Usage
let result = Service::builder()
    .dependency(dep)
    .build()
    .map_err(|e| format!("Failed to build service: {}", e))?;
```

### Async Initialization
```rust
#[builder(async_init)]
struct Service {
    repo: Arc<dyn Repository>,
}

impl Service {
    async fn init(&self) -> Result<(), Error> {
        self.repo.connect().await?;
        Ok(())
    }
}
```

## Best Practices

1. **Use Arc for Shared Services**
```rust
#[builder]
struct AppState {
    services: Arc<AppServices>,
}
```

2. **Group Related Services**
```rust
#[builder]
struct DatabaseServices {
    user_repo: Arc<dyn UserRepository>,
    post_repo: Arc<dyn PostRepository>,
}
```

3. **Clear Error Handling**
```rust
let service = Service::builder()
    .dependency(dep)
    .build()
    .expect("Failed to build service: missing dependency");
```

## Performance Considerations

The builder pattern in Rust has several performance advantages:

1. Zero runtime overhead for dependency resolution
2. No reflection or dynamic dispatch (unless explicitly used with trait objects)
3. Smaller binary size compared to DI frameworks
4. Better optimization opportunities for the compiler

## Why Not Traditional DI?

1. **Runtime Overhead**: Traditional DI containers need to resolve dependencies at runtime
2. **Type Erasure**: Many DI solutions rely heavily on type erasure and runtime checks
3. **Ownership Complexity**: DI frameworks often struggle with Rust's ownership rules
4. **Hidden Dependencies**: Dependencies are often hidden in container configuration
5. **Runtime Failures**: Many dependency issues only surface at runtime

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

--------------------
All commit messages generated by [opencommit](https://github.com/di-sukharev/opencommit)