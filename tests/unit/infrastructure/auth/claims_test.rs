//! Tests for JWT claims
//!
//! Migrated from src/infrastructure/auth/claims.rs inline tests.

use mcp_context_browser::infrastructure::auth::{Claims, HashVersion, User, UserRole};
use mcp_context_browser::infrastructure::constants::JWT_EXPIRATION_SECS;

#[test]
fn test_claims_creation() {
    let claims = Claims::new(
        "user123".to_string(),
        "test@example.com".to_string(),
        UserRole::Developer,
        "mcp-context-browser".to_string(),
        JWT_EXPIRATION_SECS,
    );

    assert_eq!(claims.sub, "user123");
    assert_eq!(claims.email, "test@example.com");
    assert_eq!(claims.role, UserRole::Developer);
    assert!(!claims.is_expired());
    assert!(claims.remaining_secs() > 3500);
}

#[test]
fn test_user_creation() {
    let user = User::new(
        "admin".to_string(),
        "admin@example.com".to_string(),
        UserRole::Admin,
        "hash123".to_string(),
    );

    assert_eq!(user.id, "admin");
    assert_eq!(user.hash_version, HashVersion::Argon2id);
}

#[test]
fn test_user_with_bcrypt() {
    let user = User::with_bcrypt_hash(
        "legacy".to_string(),
        "legacy@example.com".to_string(),
        UserRole::Viewer,
        "$2b$10$...".to_string(),
    );

    assert_eq!(user.hash_version, HashVersion::Bcrypt);
}
