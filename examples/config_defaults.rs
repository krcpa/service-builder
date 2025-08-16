use service_builder::builder;
use std::time::Duration;

#[builder]
struct DatabaseConfig {
    // Required field - no default
    connection_string: String,
    
    // Optional with custom default
    #[builder(default = "Duration::from_secs(30)")]
    timeout: Duration,
    
    // Optional with Default trait
    #[builder(default)]
    ssl_enabled: bool,
    
    // Optional field - defaults to None
    #[builder(optional)]
    max_connections: Option<usize>,
    
    // Custom default expression
    #[builder(default = "5")]
    retry_count: u32,
}

fn main() {
    println!("=== Config with Defaults Example ===\n");
    
    // Example 1: Using build_with_defaults() with minimal configuration
    println!("1. Minimal configuration with defaults:");
    let config = DatabaseConfig::builder()
        .connection_string("postgres://localhost/mydb".to_string())
        .build_with_defaults()
        .unwrap();
    
    println!("  Connection: {}", config.connection_string);
    println!("  Timeout: {:?}", config.timeout);
    println!("  SSL Enabled: {}", config.ssl_enabled);
    println!("  Max Connections: {:?}", config.max_connections);
    println!("  Retry Count: {}\n", config.retry_count);
    
    // Example 2: Override some defaults
    println!("2. Override some defaults:");
    let config = DatabaseConfig::builder()
        .connection_string("postgres://prod/db".to_string())
        .timeout(Duration::from_secs(60))
        .ssl_enabled(true)
        .max_connections(Some(100))
        .build_with_defaults()
        .unwrap();
    
    println!("  Connection: {}", config.connection_string);
    println!("  Timeout: {:?}", config.timeout);
    println!("  SSL Enabled: {}", config.ssl_enabled);
    println!("  Max Connections: {:?}", config.max_connections);
    println!("  Retry Count: {}\n", config.retry_count);
    
    // Example 3: Using strict build() - works because non-required fields have defaults
    println!("3. Using strict build() method:");
    let config = DatabaseConfig::builder()
        .connection_string("postgres://test/db".to_string())
        .build()
        .unwrap();
    
    println!("  Connection: {}", config.connection_string);
    println!("  Timeout: {:?}", config.timeout);
    println!("  SSL Enabled: {}", config.ssl_enabled);
    println!("  Max Connections: {:?}", config.max_connections);
    println!("  Retry Count: {}\n", config.retry_count);
    
    // Example 4: Demonstrate error handling
    println!("4. Error handling - missing required field:");
    let result = DatabaseConfig::builder()
        .timeout(Duration::from_secs(10))
        .build();
    
    match result {
        Err(e) => println!("  Error: {}", e),
        Ok(_) => println!("  Unexpected success!"),
    }
}

// Example of a configuration struct where all fields have defaults
#[builder]
struct AppConfig {
    #[builder(default = "\"My App\".to_string()")]
    app_name: String,
    
    #[builder(default = "8080")]
    port: u16,
    
    #[builder(default)]
    debug_mode: bool,
    
    #[builder(optional)]
    custom_header: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_all_defaults_config() {
        // Can build with no arguments when all fields have defaults
        let config = AppConfig::builder()
            .build()
            .unwrap();
        
        assert_eq!(config.app_name, "My App");
        assert_eq!(config.port, 8080);
        assert_eq!(config.debug_mode, false);
        assert_eq!(config.custom_header, None);
    }
}