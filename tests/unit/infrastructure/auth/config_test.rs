//! Tests for auth configuration
//!
//! Migrated from src/infrastructure/auth/config.rs inline tests.

use mcp_context_browser::infrastructure::auth::AuthConfig;

#[test]
fn test_default_config_has_warnings_when_enabled() {
    let config = AuthConfig {
        enabled: true,
        ..Default::default()
    };

    let warnings = config.validate_for_production();

    // Should have warnings for default credentials
    assert!(
        !warnings.is_empty(),
        "Expected security warnings for default config"
    );

    // Should include JWT secret warning
    assert!(
        warnings
            .iter()
            .any(|w| w.code == "DEFAULT_JWT_SECRET" || w.code == "JWT_SECRET_TOO_SHORT"),
        "Expected JWT secret warning"
    );
}

#[test]
fn test_disabled_config_has_no_warnings() {
    let config = AuthConfig::default();
    assert!(!config.enabled);

    let warnings = config.validate_for_production();
    assert!(warnings.is_empty(), "Disabled auth should have no warnings");
}

#[test]
fn test_bypass_paths() {
    let config = AuthConfig::default();

    assert!(config.should_bypass("/api/health"));
    assert!(config.should_bypass("/api/context/metrics"));
    assert!(!config.should_bypass("/api/search"));
}

#[test]
fn test_bypass_wildcard() {
    let mut config = AuthConfig::default();
    config.bypass_paths.push("/public/*".to_string());

    assert!(config.should_bypass("/public/docs"));
    assert!(config.should_bypass("/public/images/logo.png"));
    assert!(!config.should_bypass("/private/data"));
}
