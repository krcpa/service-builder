# service-builder

[![Crates.io](https://img.shields.io/crates/v/service-builder.svg)](https://crates.io/crates/service-builder)
[![Documentation](https://docs.rs/service-builder/badge.svg)](https://docs.rs/service-builder)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightweight, type-safe service construction library for Rust that leverages the builder pattern to provide a more idiomatic alternative to traditional dependency injection.

## Features

- 🔒 **Type-safe dependency injection** at compile time
- 🚀 **Zero runtime overhead** - everything is checked at compile time
- 🛠️ **Automatic builder implementation** via proc-macros
- 📦 **Field-level getters and setters** with attribute control
- ⚡ **Zero-cost abstractions** - no runtime reflection or dynamic dispatch
- 🔍 **Comprehensive error handling** with descriptive messages

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
service-builder = "0.2.0"
```

### Basic Usage with Builder Pattern

```rust
use service_builder::prelude::*;
use std::sync::Arc;

#[builder]
struct UserService {
    repository: Arc<dyn UserRepository>,
    cache: Arc<dyn Cache>,
}

let user_service = UserService::builder()
    .repository(user_repo)
    .cache(cache)
    .build()?;
```

### Using Getters and Setters

You can add getter and setter methods to your fields using attributes:

```rust
#[builder]
struct Config {
    #[builder(getter)]  // Generates get_api_key()
    api_key: String,
    
    #[builder(setter)]  // Generates set_timeout()
    timeout: Duration,
    
    #[builder(getter, setter)]  // Generates both
    max_retries: u32,
}

let mut config = Config::builder()
    .api_key("secret".to_string())
    .timeout(Duration::from_secs(30))
    .max_retries(3)
    .build()?;

// Use generated getter
assert_eq!(config.get_api_key(), &"secret".to_string());

// Use generated setter
config.set_max_retries(5);
```

### Composing Services

```rust
#[builder]
struct AppServices {
    #[builder(getter)]  // Access services via getters
    user_service: Arc<UserService>,
    post_service: Arc<PostService>,
}

let app_services = AppServices::builder()
    .user_service(Arc::new(user_service))
    .post_service(Arc::new(post_service))
    .build()?;

// Access services using generated getters
let user_service = app_services.get_user_service();
```

## Builder Pattern vs Traditional DI

### Advantages of Builder Pattern

1. **Type Safety**: All dependencies are checked at compile time
2. **Zero Runtime Cost**: No reflection or dynamic dispatch overhead
3. **Ownership Control**: Works naturally with Rust's ownership system
4. **Explicit Dependencies**: Dependencies are clearly visible in the code
5. **Flexible Access**: Optional getter/setter generation for fine-grained control

### Disadvantages of Traditional DI

1. **Runtime Overhead**: Container resolution and type checking at runtime
2. **Safety Issues**: Potential runtime panics from missing dependencies
3. **Ownership Complexity**: DI frameworks often struggle with Rust's ownership rules
4. **Hidden Dependencies**: Dependencies are often hidden in container configuration
5. **Runtime Failures**: Many dependency issues only surface at runtime

## Contributing

We welcome contributions! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.