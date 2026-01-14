//! Tests for auth middleware
//!
//! Migrated from src/infrastructure/auth/middleware.rs inline tests.

use mcp_context_browser::infrastructure::auth::middleware::{ClaimsExtractor, RequirePermission};
use mcp_context_browser::infrastructure::auth::{Claims, Permission, UserRole};
use mcp_context_browser::infrastructure::constants::JWT_EXPIRATION_SECS;

#[test]
fn test_claims_extractor() {
    let claims = Claims::new(
        "user1".to_string(),
        "user@example.com".to_string(),
        UserRole::Developer,
        "test".to_string(),
        JWT_EXPIRATION_SECS,
    );

    let extractor = ClaimsExtractor(claims.clone());
    assert_eq!(extractor.sub, "user1");
    assert_eq!(extractor.email, "user@example.com");
}

#[test]
fn test_require_permission() {
    let claims = Claims::new(
        "user1".to_string(),
        "user@example.com".to_string(),
        UserRole::Developer,
        "test".to_string(),
        JWT_EXPIRATION_SECS,
    );

    assert!(RequirePermission::check(
        &claims,
        &Permission::IndexCodebase
    ));
    assert!(!RequirePermission::check(&claims, &Permission::ManageUsers));
}
