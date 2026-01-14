//! JWT Claims structure and validation
//!
//! Defines the token payload structure for authentication.

use super::roles::UserRole;
use crate::infrastructure::utils::TimeUtils;
use serde::{Deserialize, Serialize};

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// User email
    pub email: String,
    /// User role
    pub role: UserRole,
    /// Issued at timestamp
    pub iat: u64,
    /// Expiration timestamp
    pub exp: u64,
    /// Issuer
    pub iss: String,
}

impl Claims {
    /// Create new claims for a user
    pub fn new(
        user_id: String,
        email: String,
        role: UserRole,
        issuer: String,
        expiration_secs: u64,
    ) -> Self {
        let now = TimeUtils::now_unix_secs();

        Self {
            sub: user_id,
            email,
            role,
            iat: now,
            exp: now + expiration_secs,
            iss: issuer,
        }
    }

    /// Check if the token has expired
    pub fn is_expired(&self) -> bool {
        self.exp < TimeUtils::now_unix_secs()
    }

    /// Get remaining validity in seconds (0 if expired)
    pub fn remaining_secs(&self) -> u64 {
        self.exp.saturating_sub(TimeUtils::now_unix_secs())
    }
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct User {
    /// Unique user ID
    pub id: String,
    /// User email
    pub email: String,
    /// User role
    pub role: UserRole,
    /// Password hash (Argon2id or bcrypt for migration)
    #[serde(skip)]
    pub password_hash: String,
    /// Hash algorithm version (for migration: "bcrypt" or "argon2id")
    #[serde(skip)]
    pub hash_version: HashVersion,
    /// When user was created
    pub created_at: u64,
    /// When user was last active
    pub last_active: u64,
}

/// Password hash algorithm version for migration support
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HashVersion {
    /// Legacy bcrypt hashes (migration path)
    Bcrypt,
    /// Modern Argon2id hashes (preferred)
    #[default]
    Argon2id,
}

impl User {
    /// Create a new user with default timestamps
    pub fn new(id: String, email: String, role: UserRole, password_hash: String) -> Self {
        Self {
            id,
            email,
            role,
            password_hash,
            hash_version: HashVersion::Argon2id,
            created_at: TimeUtils::now_unix_secs(),
            last_active: 0,
        }
    }

    /// Create a user with bcrypt hash (for migration)
    pub fn with_bcrypt_hash(
        id: String,
        email: String,
        role: UserRole,
        password_hash: String,
    ) -> Self {
        let mut user = Self::new(id, email, role, password_hash);
        user.hash_version = HashVersion::Bcrypt;
        user
    }

    /// Update last active timestamp
    pub fn touch(&mut self) {
        self.last_active = TimeUtils::now_unix_secs();
    }
}
