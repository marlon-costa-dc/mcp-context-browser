//! Tests for auth service
//!
//! Migrated from src/infrastructure/auth/service.rs inline tests.

use mcp_context_browser::infrastructure::auth::password;
use mcp_context_browser::infrastructure::auth::{
    AuthConfig, AuthService, Permission, User, UserRole,
};

fn test_config() -> AuthConfig {
    let mut config = AuthConfig::new("test-secret-at-least-32-bytes-long".to_string(), 3600, true);

    // Add test user with known password
    let hash = password::hash_password("testpass").unwrap();
    config.add_user(User::new(
        "test".to_string(),
        "test@example.com".to_string(),
        UserRole::Developer,
        hash,
    ));

    config
}

#[test]
fn test_authenticate_success() {
    let service = AuthService::new(test_config());

    let token = service.authenticate("test@example.com", "testpass");
    assert!(token.is_ok(), "Authentication should succeed");
}

#[test]
fn test_authenticate_wrong_password() {
    let service = AuthService::new(test_config());

    let result = service.authenticate("test@example.com", "wrongpass");
    assert!(result.is_err(), "Authentication should fail");
}

#[test]
fn test_authenticate_unknown_user() {
    let service = AuthService::new(test_config());

    let result = service.authenticate("unknown@example.com", "testpass");
    assert!(result.is_err(), "Authentication should fail");
}

#[test]
fn test_validate_token() {
    let service = AuthService::new(test_config());

    let token = service
        .authenticate("test@example.com", "testpass")
        .unwrap();
    let claims = service.validate_token(&token);

    assert!(claims.is_ok());
    let claims = claims.unwrap();
    assert_eq!(claims.email, "test@example.com");
    assert_eq!(claims.role, UserRole::Developer);
}

#[test]
fn test_disabled_auth() {
    let mut config = test_config();
    config.enabled = false;

    let service = AuthService::new(config);

    assert!(!service.is_enabled());
    assert!(service
        .authenticate("test@example.com", "testpass")
        .is_err());
}

#[test]
fn test_check_permission() {
    let service = AuthService::new(test_config());

    let token = service
        .authenticate("test@example.com", "testpass")
        .unwrap();
    let claims = service.validate_token(&token).unwrap();

    // Developer should have indexing permission
    assert!(service.check_permission(&claims, &Permission::IndexCodebase));
    // Developer should NOT have user management permission
    assert!(!service.check_permission(&claims, &Permission::ManageUsers));
}

#[test]
fn test_bypass_paths() {
    let service = AuthService::new(test_config());

    assert!(service.should_bypass("/api/health"));
    assert!(!service.should_bypass("/api/search"));
}
