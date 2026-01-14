//! Tests for API key management
//!
//! Migrated from src/infrastructure/auth/api_keys.rs inline tests.

use mcp_context_browser::infrastructure::auth::{
    api_keys::{ApiKey, ApiKeyStore, API_KEY_PREFIX},
    User, UserRole,
};

fn test_user() -> User {
    User::new(
        "test".to_string(),
        "test@example.com".to_string(),
        UserRole::Developer,
        "hash".to_string(),
    )
}

#[test]
fn test_create_and_validate_key() {
    let store = ApiKeyStore::new();
    let user = test_user();

    let (key_plaintext, api_key) = store
        .create_key("Test Key".to_string(), &user, None)
        .unwrap();

    assert!(key_plaintext.starts_with(API_KEY_PREFIX));
    assert!(api_key.active);
    assert!(api_key.expires_at.is_none());

    // Validate the key
    let claims = store.validate_key(&key_plaintext).unwrap();
    assert_eq!(claims.email, "test@example.com");
    assert_eq!(claims.role, UserRole::Developer);
}

#[test]
fn test_invalid_key() {
    let store = ApiKeyStore::new();

    let result = store.validate_key("invalid_key");
    assert!(result.is_err());
}

#[test]
fn test_revoke_key() {
    let store = ApiKeyStore::new();
    let user = test_user();

    let (key_plaintext, api_key) = store
        .create_key("Test Key".to_string(), &user, None)
        .unwrap();

    // Revoke the key
    store.revoke_key(&api_key.id).unwrap();

    // Key should no longer be valid
    let result = store.validate_key(&key_plaintext);
    assert!(result.is_err());
}

#[test]
fn test_expired_key() {
    let mut api_key = ApiKey {
        id: "test_key".to_string(),
        name: "Test".to_string(),
        user_email: "test@example.com".to_string(),
        role: UserRole::Developer,
        key_hash: String::new(),
        created_at: 0,
        expires_at: Some(1), // Expired in 1970
        last_used: 0,
        active: true,
    };

    assert!(api_key.is_expired());
    assert!(!api_key.is_valid());

    // Key with no expiration
    api_key.expires_at = None;
    assert!(!api_key.is_expired());
}
