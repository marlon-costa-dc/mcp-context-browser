//! Tests for auth roles
//!
//! Migrated from src/infrastructure/auth/roles.rs inline tests.

use mcp_context_browser::infrastructure::auth::{Permission, UserRole};

#[test]
fn test_admin_has_all_permissions() {
    let admin = UserRole::Admin;
    assert!(admin.has_permission(&Permission::IndexCodebase));
    assert!(admin.has_permission(&Permission::SearchCodebase));
    assert!(admin.has_permission(&Permission::ViewMetrics));
    assert!(admin.has_permission(&Permission::ManageUsers));
    assert!(admin.has_permission(&Permission::ManageSystem));
}

#[test]
fn test_guest_has_minimal_permissions() {
    let guest = UserRole::Guest;
    assert!(!guest.has_permission(&Permission::IndexCodebase));
    assert!(!guest.has_permission(&Permission::SearchCodebase));
    assert!(guest.has_permission(&Permission::ViewMetrics));
    assert!(!guest.has_permission(&Permission::ManageUsers));
    assert!(!guest.has_permission(&Permission::ManageSystem));
}

#[test]
fn test_role_assignment() {
    let admin = UserRole::Admin;
    let developer = UserRole::Developer;
    let viewer = UserRole::Viewer;

    assert!(admin.can_assign(&UserRole::Developer));
    assert!(admin.can_assign(&UserRole::Admin));
    assert!(developer.can_assign(&UserRole::Viewer));
    assert!(!developer.can_assign(&UserRole::Admin));
    assert!(!viewer.can_assign(&UserRole::Guest));
}

#[test]
fn test_role_hierarchy() {
    assert!(UserRole::Admin.level() > UserRole::Developer.level());
    assert!(UserRole::Developer.level() > UserRole::Viewer.level());
    assert!(UserRole::Viewer.level() > UserRole::Guest.level());
}
