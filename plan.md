# service-builder Enhancement Plan

## Current State
service-builder works well for dependency injection patterns but has limitations for configuration structs that need default values and optional fields.

## Enhancement Goals
Make service-builder suitable for both dependency injection AND configuration patterns by adding support for defaults and optional fields.

## Planned Features

### 1. Default Value Support
Add `#[builder(default = "expression")]` attribute support:

```rust
#[builder]
struct CacheConfig {
    #[builder(default = "Duration::from_secs(3600)")]
    default_ttl: Option<Duration>,
    
    #[builder(default = "10_000")]
    max_entries: Option<usize>,
    
    #[builder(default)] // Uses Default::default()
    compression: bool,
    
    #[builder(default = "Duration::from_secs(5)")]
    connection_timeout: Duration,
}
```

**Implementation Plan:**
- Extend `field_attributes.rs` to parse `default` attribute
- Modify builder generation to use defaults for unset fields
- Support both literal expressions and `Default::default()`

### 2. Optional Field Handling
Add `#[builder(optional)]` for fields that should default to `None`:

```rust
#[builder]
struct CacheConfig {
    #[builder(optional)] // Defaults to None if not set
    default_ttl: Option<Duration>,
    
    #[builder(optional)]
    max_entries: Option<usize>,
}
```

**Implementation Plan:**
- Add `optional` attribute parsing
- Generate builder code that doesn't require these fields
- Automatically set to `None` if not provided

### 3. Build Methods Variants
Provide multiple build strategies:

```rust
impl CacheConfigBuilder {
    // Current behavior - fails if required fields missing
    pub fn build(self) -> Result<CacheConfig, BuildError> { ... }
    
    // New - uses defaults, never fails for config structs
    pub fn build_with_defaults(self) -> CacheConfig { ... }
    
    // New - validates all fields are set or have defaults
    pub fn build_validated(self) -> Result<CacheConfig, BuildError> { ... }
}
```

### 4. Custom Builder Methods Support
Allow custom convenience methods alongside generated ones:

```rust
// Generated builder gets custom methods via impl block
impl CacheConfigBuilder {
    // Custom convenience methods
    pub fn no_default_ttl(self) -> Self {
        self.default_ttl(None)
    }
    
    pub fn unlimited_entries(self) -> Self {
        self.max_entries(None)
    }
    
    pub fn enable_compression(self, threshold: usize) -> Self {
        self.compression(true).compression_threshold(threshold)
    }
}
```

### 5. Field Validation Support
Add validation attributes:

```rust
#[builder]
struct CacheConfig {
    #[builder(validate = "validate_ttl")]
    default_ttl: Option<Duration>,
    
    #[builder(validate = "|v| v > 0", message = "Max entries must be positive")]
    max_entries: Option<usize>,
}

fn validate_ttl(ttl: &Option<Duration>) -> Result<(), String> {
    // Custom validation logic
}
```

## Implementation Priority

### Phase 1: Core Default Support
1. `#[builder(default)]` using `Default::default()`
2. `#[builder(default = "expression")]` with literal expressions
3. `build_with_defaults()` method

### Phase 2: Optional Fields
1. `#[builder(optional)]` attribute
2. Automatic `Option<T>` handling
3. Better error messages

### Phase 3: Enhanced Builder Methods
1. `build_validated()` method
2. Custom method support documentation
3. Integration examples

### Phase 4: Validation (Future)
1. `#[builder(validate)]` attribute
2. Custom validation functions
3. Validation error aggregation

## Success Criteria

After implementation, this should work seamlessly:

```rust
#[builder]
struct CacheConfig {
    #[builder(optional)]
    default_ttl: Option<Duration>,
    
    #[builder(optional)]
    max_entries: Option<usize>,
    
    #[builder(default = "Duration::from_secs(5)")]
    connection_timeout: Duration,
    
    #[builder(default)]
    compression: bool,
}

// Usage 1: Minimal config
let config = CacheConfig::builder()
    .build_with_defaults(); // Uses all defaults

// Usage 2: Partial config  
let config = CacheConfig::builder()
    .max_entries(Some(1000))
    .build_with_defaults();

// Usage 3: Full validation
let config = CacheConfig::builder()
    .default_ttl(Some(Duration::from_secs(3600)))
    .max_entries(Some(1000))
    .build_validated()?; // Ensures all fields handled
```

## Testing Strategy

1. **Unit Tests**: Each new attribute and feature
2. **Integration Tests**: Real configuration structs
3. **Regression Tests**: Ensure existing DI patterns still work
4. **Documentation Tests**: All examples in docs compile and run

## Migration Path

1. **Backward Compatible**: All existing code continues to work
2. **Opt-in Features**: New attributes are optional
3. **Clear Documentation**: When to use each pattern
4. **Migration Examples**: Show how to convert manual builders