//! Authentication utilities for MCP server
//!
//! This module contains authentication-related functionality extracted
//! from the main server implementation to improve separation of concerns.

use crate::domain::error::Result;
use crate::infrastructure::auth::{AuthServiceInterface, Claims, Permission};

/// Authentication utilities for MCP server tools
pub struct AuthHandler {
    auth_service: std::sync::Arc<dyn AuthServiceInterface>,
}

impl AuthHandler {
    /// Create a new authentication handler from Arc<dyn AuthServiceInterface>
    pub fn new_from_arc(auth_service: std::sync::Arc<dyn AuthServiceInterface>) -> Self {
        Self { auth_service }
    }

    /// Get the authentication service
    pub fn auth_service(&self) -> std::sync::Arc<dyn AuthServiceInterface> {
        std::sync::Arc::clone(&self.auth_service)
    }

    /// Check authentication and permissions for a request
    ///
    /// Returns Ok(Some(claims)) if authentication succeeds and permissions are granted
    /// Returns Ok(None) if authentication is disabled
    /// Returns Err if authentication fails or permissions are insufficient
    pub fn check_auth(
        &self,
        token: Option<&String>,
        required_permission: &Permission,
    ) -> Result<Option<Claims>> {
        if !self.auth_service.is_enabled() {
            return Ok(None); // Auth disabled, allow all requests
        }

        let Some(token) = token else {
            return Err(crate::domain::error::Error::generic(
                "Authentication required",
            ));
        };

        let claims = self.auth_service.validate_token(token)?;
        if !self
            .auth_service
            .check_permission(&claims, required_permission)
        {
            return Err(crate::domain::error::Error::generic(
                "Insufficient permissions",
            ));
        }

        Ok(Some(claims))
    }

    /// Check if authentication is enabled
    pub fn is_enabled(&self) -> bool {
        self.auth_service.is_enabled()
    }
}
