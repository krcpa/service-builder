# CLAUDE.md — service-builder

## What is service-builder?
A lightweight, type-safe service construction library for Rust that leverages the builder pattern for both dependency injection and configuration management. Designed as an idiomatic alternative to traditional DI frameworks.

## Core Philosophy
- **Compile-Time Safety**: All dependencies checked at build time
- **Zero Runtime Overhead**: No reflection or dynamic dispatch  
- **Rust-Native**: Works with ownership system, not against it
- **Unified Pattern**: Single approach for services AND configuration

## Quick Usage

### Dependency Injection Pattern
```rust
use service_builder::builder;

#[builder]
struct UserService {
    repository: Arc<dyn UserRepository>,
    cache: Arc<dyn Cache>,
}

// Usage - all fields required
let service = UserService::builder()
    .repository(user_repo)
    .cache(cache)
    .build()?; // Returns Result<UserService, BuildError>
```

### Configuration Pattern
```rust
#[builder]
struct CacheConfig {
    #[builder(optional)]
    default_ttl: Option<Duration>,
    
    #[builder(optional)]  
    max_entries: Option<usize>,
    
    #[builder(default = "Duration::from_secs(5)")]
    connection_timeout: Duration,
    
    #[builder(default)]  // Uses Default::default()
    compression: bool,
}

// Usage - defaults provided
let config = CacheConfig::builder()
    .max_entries(Some(1000))
    .build_with_defaults(); // Never fails, uses defaults
```

### With Getters and Setters
```rust
#[builder]
struct Config {
    #[builder(getter)]
    api_key: String,
    
    #[builder(setter)]
    timeout: Duration,
    
    #[builder(getter, setter)]
    retry_count: u32,
}

let mut config = Config::builder()
    .api_key("secret".to_string())
    .timeout(Duration::from_secs(30))
    .retry_count(3)
    .build()?;

// Use generated methods
let key = config.get_api_key();
config.set_retry_count(5);
```

## Attribute Reference

### Field Attributes

#### `#[builder(optional)]`
Field defaults to `None` if not provided. Only works with `Option<T>` types.
```rust
#[builder(optional)]
cache_size: Option<usize>,  // Defaults to None
```

#### `#[builder(default)]`  
Field uses `Default::default()` if not provided.
```rust
#[builder(default)]
enabled: bool,  // Defaults to false
```

#### `#[builder(default = "expression")]`
Field uses custom default expression if not provided.
```rust
#[builder(default = "Duration::from_secs(30)")]
timeout: Duration,
```

#### `#[builder(getter)]`
Generates `get_field_name()` method returning `&FieldType`.
```rust
#[builder(getter)]
database_url: String,  // Generates get_database_url() -> &String
```

#### `#[builder(setter)]`
Generates `set_field_name(value: FieldType)` method.
```rust
#[builder(setter)]
retry_count: u32,  // Generates set_retry_count(value: u32)
```

## Generated API

### Builder Methods
- `StructName::builder()` → `StructNameBuilder`
- `builder.field_name(value)` → `StructNameBuilder` (fluent)
- `builder.build()` → `Result<StructName, BuildError>` (strict)
- `builder.build_with_defaults()` → `StructName` (uses defaults, never fails)
- `builder.build_validated()` → `Result<StructName, BuildError>` (ensures all fields handled)

### Getter Methods (with `#[builder(getter)]`)
- `instance.get_field_name()` → `&FieldType`

### Setter Methods (with `#[builder(setter)]`)  
- `instance.set_field_name(value: FieldType)`

## Build Strategies

### `build()` - Strict (Default)
Returns error if required fields missing. Best for dependency injection.
```rust
UserService::builder()
    .repository(repo)
    // Missing cache field
    .build(); // Error: MissingDependency("cache")
```

### `build_with_defaults()` - Permissive
Uses defaults for missing fields, never fails. Best for configuration.
```rust
CacheConfig::builder()
    .max_size(Some(1000))
    .build_with_defaults(); // OK: uses defaults for other fields
```

### `build_validated()` - Verified
Ensures all fields are either set or have defaults. Best for critical config.
```rust
DatabaseConfig::builder()
    .connection_string("...")
    .build_validated()?; // OK: all fields covered
```

## Error Handling

`BuildError` variants:
- `MissingDependency(field_name)` - Required field not set
- `InitializationError(message)` - Custom validation failed  
- `ConfigurationError(message)` - Invalid configuration
- `BuildFailed(message)` - General build failure

## Usage Patterns

### 1. Service Construction (Dependency Injection)
Use when building services with required dependencies:
```rust
#[builder]
struct EmailService {
    smtp_client: Arc<SmtpClient>,
    template_engine: Arc<TemplateEngine>,
    #[builder(getter)]
    config: EmailConfig,
}
```

### 2. Configuration Objects
Use when building config with sensible defaults:
```rust
#[builder] 
struct ServerConfig {
    #[builder(optional)]
    host: Option<String>,
    
    #[builder(default = "8080")]
    port: u16,
    
    #[builder(default)]
    debug_mode: bool,
}
```

### 3. Hybrid Patterns
Combine required dependencies with optional configuration:
```rust
#[builder]
struct DatabaseService {
    // Required dependency
    connection_pool: Arc<ConnectionPool>,
    
    // Optional configuration  
    #[builder(optional)]
    max_connections: Option<usize>,
    
    #[builder(default = "Duration::from_secs(30)")]
    timeout: Duration,
}
```

## Custom Builder Methods

You can add custom convenience methods to generated builders:

```rust
#[builder]
struct CacheConfig {
    #[builder(optional)]
    max_entries: Option<usize>,
    #[builder(default)]
    compression: bool,
    #[builder(default = "1024")]
    compression_threshold: usize,
}

impl CacheConfigBuilder {
    pub fn unlimited_entries(self) -> Self {
        self.max_entries(None)
    }
    
    pub fn enable_compression(self, threshold: usize) -> Self {
        self.compression(true).compression_threshold(threshold)
    }
}
```

## Integration with elif.rs

Perfect for elif.rs module and service construction:

```rust
#[builder]
struct UserModule {
    // Required dependencies
    repository: Arc<dyn UserRepository>,
    cache: Arc<dyn Cache>,
    
    // Optional configuration
    #[builder(getter)]
    #[builder(default = "UserConfig::default()")]
    config: UserConfig,
}

// Application composition
let user_module = UserModule::builder()
    .repository(Arc::new(PostgresUserRepo::new(pool)))
    .cache(Arc::new(RedisCache::new(client)))
    .build_with_defaults();
```

## Development Commands

```bash
# Run tests
cargo test
cargo test --features all

# Check examples
cargo run --example basic
cargo run --example config

# Format and lint
cargo fmt
cargo clippy
```

---
**Status**: Enhanced for configuration patterns
**See**: `plan.md` for development roadmap