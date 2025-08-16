use service_builder::builder;
use std::time::Duration;

#[builder]
struct CacheConfig {
    #[builder(default = "Duration::from_secs(3600)")]
    default_ttl: Duration,
    
    #[builder(default = "10_000")]
    max_entries: usize,
    
    #[builder(default)]
    compression: bool,
    
    #[builder(optional)]
    custom_name: Option<String>,
    
    #[builder(default = "Duration::from_secs(5)")]
    connection_timeout: Duration,
    
    // Required field - no default
    cache_dir: String,
}

#[test]
fn test_build_with_defaults_all_defaults() {
    let config = CacheConfig::builder()
        .cache_dir("/tmp/cache".to_string())
        .build_with_defaults()
        .unwrap();
    
    assert_eq!(config.default_ttl, Duration::from_secs(3600));
    assert_eq!(config.max_entries, 10_000);
    assert_eq!(config.compression, false); // bool default
    assert_eq!(config.custom_name, None);
    assert_eq!(config.connection_timeout, Duration::from_secs(5));
    assert_eq!(config.cache_dir, "/tmp/cache");
}

#[test]
fn test_build_with_defaults_partial_override() {
    let config = CacheConfig::builder()
        .cache_dir("/var/cache".to_string())
        .max_entries(50_000)
        .compression(true)
        .custom_name(Some("my-cache".to_string()))
        .build_with_defaults()
        .unwrap();
    
    assert_eq!(config.default_ttl, Duration::from_secs(3600)); // uses default
    assert_eq!(config.max_entries, 50_000); // overridden
    assert_eq!(config.compression, true); // overridden
    assert_eq!(config.custom_name, Some("my-cache".to_string())); // overridden
    assert_eq!(config.connection_timeout, Duration::from_secs(5)); // uses default
}

#[test]
fn test_strict_build_missing_required() {
    let result = CacheConfig::builder()
        .max_entries(1000)
        .build();
    
    assert!(result.is_err());
    match result {
        Err(service_builder::error::BuildError::MissingDependency(field)) => {
            assert_eq!(field, "cache_dir");
        }
        _ => panic!("Expected MissingDependency error"),
    }
}

#[test]
fn test_strict_build_with_defaults_provided() {
    let config = CacheConfig::builder()
        .cache_dir("/tmp".to_string())
        .build()
        .unwrap();
    
    // Defaults should still be applied for non-required fields
    assert_eq!(config.default_ttl, Duration::from_secs(3600));
    assert_eq!(config.max_entries, 10_000);
    assert_eq!(config.compression, false);
    assert_eq!(config.custom_name, None);
}

#[builder]
struct ConfigWithOnlyDefaults {
    #[builder(default = "true")]
    enabled: bool,
    
    #[builder(default = "\"default\".to_string()")]
    name: String,
    
    #[builder(optional)]
    description: Option<String>,
}

#[test]
fn test_all_fields_have_defaults() {
    let config = ConfigWithOnlyDefaults::builder()
        .build_with_defaults()
        .unwrap();
    
    assert_eq!(config.enabled, true);
    assert_eq!(config.name, "default");
    assert_eq!(config.description, None);
}

#[test]
fn test_build_method_with_all_defaults() {
    // The regular build() should also work when all fields have defaults
    let config = ConfigWithOnlyDefaults::builder()
        .build()
        .unwrap();
    
    assert_eq!(config.enabled, true);
    assert_eq!(config.name, "default");
    assert_eq!(config.description, None);
}

#[test]
fn test_build_with_defaults_missing_required_field() {
    // build_with_defaults should still fail for required fields without defaults
    let result = CacheConfig::builder()
        .max_entries(1000)
        // Missing cache_dir which is required and has no default
        .build_with_defaults();
    
    assert!(result.is_err());
    match result {
        Err(service_builder::error::BuildError::MissingDependency(field)) => {
            assert_eq!(field, "cache_dir");
        }
        _ => panic!("Expected MissingDependency error"),
    }
}