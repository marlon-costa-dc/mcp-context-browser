//! Unit tests for admin configuration and API types
//!
//! Tests for AdminConfig security-first design and validation.

use mcp_context_browser::server::admin::config::AdminConfig;

#[test]
fn test_insecure_password_detection() {
    let config = AdminConfig {
        enabled: true,
        username: "admin".to_string(),
        password: "admin".to_string(),
        jwt_secret: "a".repeat(32),
        jwt_expiration: 3600,
    };

    let result = config.validate_for_production();
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Password is insecure"));
}

#[test]
fn test_insecure_jwt_secret_detection() {
    let config = AdminConfig {
        enabled: true,
        username: "admin".to_string(),
        password: "securepassword123".to_string(),
        jwt_secret: "default-jwt-secret-change-in-production".to_string(),
        jwt_expiration: 3600,
    };

    let result = config.validate_for_production();
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("JWT secret is insecure"));
}

#[test]
fn test_short_password_rejected() {
    let config = AdminConfig {
        enabled: true,
        username: "admin".to_string(),
        password: "short".to_string(),
        jwt_secret: "a".repeat(32),
        jwt_expiration: 3600,
    };

    let result = config.validate_for_production();
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("at least 8 characters"));
}

#[test]
fn test_short_jwt_secret_rejected() {
    let config = AdminConfig {
        enabled: true,
        username: "admin".to_string(),
        password: "securepassword123".to_string(),
        jwt_secret: "tooshort".to_string(),
        jwt_expiration: 3600,
    };

    let result = config.validate_for_production();
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("at least 32 characters"));
}

#[test]
fn test_valid_config_accepted() {
    let config = AdminConfig {
        enabled: true,
        username: "admin".to_string(),
        password: "securepassword123".to_string(),
        jwt_secret: "a".repeat(32),
        jwt_expiration: 3600,
    };

    let result = config.validate_for_production();
    assert!(result.is_ok());
}

#[test]
fn test_disabled_config_skips_validation() {
    let config = AdminConfig::disabled();
    let result = config.validate_for_production();
    assert!(result.is_ok());
}

#[test]
fn test_default_is_disabled() {
    let config = AdminConfig::default();
    assert!(!config.enabled);
}

#[test]
fn test_security_warnings() {
    let config = AdminConfig {
        enabled: true,
        username: "admin".to_string(),
        password: "admin".to_string(),
        jwt_secret: "default-jwt-secret-change-in-production".to_string(),
        jwt_expiration: 3600,
    };

    let warnings = config.security_warnings();
    assert_eq!(warnings.len(), 2);
    assert!(warnings[0].contains("CRITICAL"));
    assert!(warnings[1].contains("CRITICAL"));
}
