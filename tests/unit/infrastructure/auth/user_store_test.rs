//! Tests for user store management
//!
//! Migrated from src/infrastructure/auth/user_store.rs inline tests.

use mcp_context_browser::infrastructure::auth::user_store::{StoredUser, UserStore};
use mcp_context_browser::infrastructure::auth::{HashVersion, User, UserRole};
use tempfile::TempDir;

// Constants for tests (matching src/infrastructure/auth/user_store.rs)
// Note: Import from the actual source to ensure consistency
use mcp_context_browser::infrastructure::auth::user_store::MIN_JWT_SECRET_LENGTH;
const GENERATED_PASSWORD_LENGTH: usize = 24;

#[test]
fn test_generate_creates_valid_store() {
    let (store, password) = UserStore::generate_new().expect("should generate");

    assert_eq!(store.users.len(), 1);
    assert_eq!(store.users[0].email, "admin@local");
    assert_eq!(store.users[0].role, UserRole::Admin);
    assert!(!store.users[0].password_hash.is_empty());
    assert!(store.users[0].password_hash.starts_with("$argon2"));
    assert_eq!(password.len(), GENERATED_PASSWORD_LENGTH);
    assert_eq!(store.jwt_secret.len(), MIN_JWT_SECRET_LENGTH);
}

#[test]
fn test_with_credentials_hashes_password() {
    let store =
        UserStore::with_credentials("test@example.com", "MyPassword123").expect("should create");

    assert_eq!(store.users.len(), 1);
    assert_eq!(store.users[0].email, "test@example.com");
    assert!(store.users[0].password_hash.starts_with("$argon2"));
}

#[tokio::test]
async fn test_save_and_load_roundtrip() {
    let temp_dir = TempDir::new().expect("temp dir");
    let path = temp_dir.path().join("users.json");

    let (original, _password) = UserStore::generate_new().expect("should generate");
    original.save(&path).await.expect("should save");

    let loaded = UserStore::load(&path)
        .await
        .expect("should load")
        .expect("should exist");

    assert_eq!(loaded.users.len(), original.users.len());
    assert_eq!(loaded.users[0].email, original.users[0].email);
    assert_eq!(loaded.jwt_secret, original.jwt_secret);
}

#[tokio::test]
async fn test_load_nonexistent_returns_none() {
    let temp_dir = TempDir::new().expect("temp dir");
    let path = temp_dir.path().join("nonexistent.json");

    let result = UserStore::load(&path).await.expect("should not error");
    assert!(result.is_none());
}

#[cfg(unix)]
#[tokio::test]
async fn test_file_permissions_are_0600() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().expect("temp dir");
    let path = temp_dir.path().join("users.json");

    let (store, _) = UserStore::generate_new().expect("should generate");
    store.save(&path).await.expect("should save");

    let metadata = std::fs::metadata(&path).expect("metadata");
    let mode = metadata.permissions().mode() & 0o777;
    assert_eq!(mode, 0o600, "File permissions should be 0600");
}

#[test]
fn test_jwt_secret_length() {
    let (store, _) = UserStore::generate_new().expect("should generate");
    assert!(
        store.jwt_secret.len() >= MIN_JWT_SECRET_LENGTH,
        "JWT secret should be at least {} chars",
        MIN_JWT_SECRET_LENGTH
    );
}

#[test]
fn test_to_user_map() {
    let (store, _) = UserStore::generate_new().expect("should generate");
    let map = store.to_user_map();

    assert_eq!(map.len(), 1);
    assert!(map.contains_key("admin@local"));

    let user = map.get("admin@local").unwrap();
    assert_eq!(user.role, UserRole::Admin);
    assert!(user.password_hash.starts_with("$argon2"));
}

#[test]
fn test_stored_user_conversion() {
    let stored = StoredUser {
        id: "test".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::Developer,
        password_hash: "$argon2id$test".to_string(),
        hash_version: "Argon2id".to_string(),
        created_at: 12345,
        last_active: 0,
    };

    let user: User = stored.clone().into();
    assert_eq!(user.id, "test");
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.hash_version, HashVersion::Argon2id);

    let back: StoredUser = (&user).into();
    assert_eq!(back.email, stored.email);
    assert_eq!(back.hash_version, "Argon2id");
}
