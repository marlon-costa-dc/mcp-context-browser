//! Tests for password hashing and verification
//!
//! Migrated from src/infrastructure/auth/password.rs inline tests.

use mcp_context_browser::infrastructure::auth::password::{
    hash_password, needs_migration, verify_password, PasswordPolicy,
};

#[test]
fn test_hash_and_verify() {
    let password = "test_password_123";
    let hash = hash_password(password).expect("hash should succeed");

    assert!(verify_password(password, &hash).expect("verify should succeed"));
    assert!(!verify_password("wrong_password", &hash).expect("verify should succeed"));
}

#[test]
fn test_bcrypt_verification() {
    // Known bcrypt hash for "admin"
    let bcrypt_hash = "$2b$10$7CJMei/BYSIj2KaM2dLq.eYSD5qv3wofVoaHiMf2vWxjGfbFPV3W";
    // Note: This is a test hash, actual verification depends on correct hash

    // Verify bcrypt detection
    assert!(needs_migration(bcrypt_hash));
}

#[test]
fn test_empty_hash_returns_false() {
    assert!(!verify_password("any_password", "").expect("should handle empty hash"));
}

#[test]
fn test_password_policy() {
    let policy = PasswordPolicy::default();

    // Valid password
    assert!(policy.validate("Password123").is_ok());

    // Too short
    assert!(policy.validate("Pwd1").is_err());

    // No uppercase
    assert!(policy.validate("password123").is_err());

    // No lowercase
    assert!(policy.validate("PASSWORD123").is_err());

    // No digit
    assert!(policy.validate("PasswordABC").is_err());
}

#[test]
fn test_needs_migration() {
    // bcrypt hashes need migration
    assert!(needs_migration("$2b$10$..."));
    assert!(needs_migration("$2a$10$..."));

    // Argon2 hashes don't need migration
    assert!(!needs_migration("$argon2id$v=19$..."));
}
