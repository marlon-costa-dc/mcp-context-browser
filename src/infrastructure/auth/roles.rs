//! Role-Based Access Control (RBAC) definitions
//!
//! Defines user roles and permissions with hierarchical access control.

use serde::{Deserialize, Serialize};

/// User roles with hierarchical permissions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub enum UserRole {
    /// Guest access - minimal permissions
    Guest,
    /// Viewer access - read-only operations
    Viewer,
    /// Developer access - indexing and search
    Developer,
    /// Admin access - full system control
    Admin,
}

impl UserRole {
    /// Check if this role has a specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        match self {
            UserRole::Admin => true, // Admin has all permissions
            UserRole::Developer => matches!(
                permission,
                Permission::IndexCodebase | Permission::SearchCodebase | Permission::ViewMetrics
            ),
            UserRole::Viewer => matches!(
                permission,
                Permission::SearchCodebase | Permission::ViewMetrics
            ),
            UserRole::Guest => matches!(permission, Permission::ViewMetrics),
        }
    }

    /// Check if this role can be assigned by another role
    pub fn can_assign(&self, target_role: &UserRole) -> bool {
        match self {
            UserRole::Admin => true,
            UserRole::Developer => matches!(target_role, UserRole::Viewer | UserRole::Guest),
            _ => false,
        }
    }

    /// Get the hierarchy level (higher = more privileges)
    pub fn level(&self) -> u8 {
        match self {
            UserRole::Guest => 0,
            UserRole::Viewer => 1,
            UserRole::Developer => 2,
            UserRole::Admin => 3,
        }
    }
}

/// System permissions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    /// Can index codebases
    IndexCodebase,
    /// Can search codebases
    SearchCodebase,
    /// Can view system metrics
    ViewMetrics,
    /// Can manage users and roles
    ManageUsers,
    /// Can configure system settings
    ManageSystem,
}

impl Permission {
    /// Get all permissions available to a role
    pub fn for_role(role: &UserRole) -> Vec<Permission> {
        match role {
            UserRole::Admin => vec![
                Permission::IndexCodebase,
                Permission::SearchCodebase,
                Permission::ViewMetrics,
                Permission::ManageUsers,
                Permission::ManageSystem,
            ],
            UserRole::Developer => vec![
                Permission::IndexCodebase,
                Permission::SearchCodebase,
                Permission::ViewMetrics,
            ],
            UserRole::Viewer => vec![Permission::SearchCodebase, Permission::ViewMetrics],
            UserRole::Guest => vec![Permission::ViewMetrics],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
