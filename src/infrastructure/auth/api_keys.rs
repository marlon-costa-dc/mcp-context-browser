//! API Key Authentication
//!
//! Provides API key-based authentication as an alternative to JWT tokens.
//! API keys are hashed using Argon2id and stored securely.
//!
//! # Storage Model
//!
//! **Current Implementation (v0.1.0)**: In-Memory
//!
//! API keys are stored in a thread-safe `RwLock<HashMap>` and are lost when the
//! server restarts. This is suitable for development and testing but NOT for
//! production deployments where key persistence is required.
//!
//! **Recommended for Production (v0.2.0+)**:
//!
//! For persistent API key storage, consider one of these approaches:
//!
//! 1. **SQLite/Database** - Store keys in a persistent database
//!    ```sql
//!    CREATE TABLE api_keys (
//!        id TEXT PRIMARY KEY,
//!        name TEXT NOT NULL,
//!        key_hash TEXT NOT NULL,
//!        user_email TEXT NOT NULL,
//!        role TEXT NOT NULL,
//!        created_at INTEGER NOT NULL,
//!        expires_at INTEGER,
//!        last_used INTEGER,
//!        active INTEGER DEFAULT 1
//!    );
//!    ```
//!
//! 2. **Encrypted File** - Store keys in an encrypted JSON/TOML file
//!    using the existing encryption infrastructure
//!
//! 3. **External Secrets Manager** - HashiCorp Vault, AWS Secrets Manager, etc.
//!
//! # Security Considerations
//!
//! - API key plaintext is ONLY returned once at creation time
//! - Keys are stored as Argon2id hashes, never in plaintext
//! - Keys can be revoked immediately via `revoke_key()`
//! - Keys support automatic expiration via `expires_at`

use super::claims::{Claims, User};
use super::password;
use super::roles::UserRole;
use crate::domain::error::{Error, Result};
use crate::infrastructure::utils::{RwLockExt, TimeUtils};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

/// API key prefix for identification
pub const API_KEY_PREFIX: &str = "mcb_";

/// API key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// Key ID (public identifier)
    pub id: String,
    /// Key name/description
    pub name: String,
    /// Associated user email
    pub user_email: String,
    /// Role granted by this key
    pub role: UserRole,
    /// Hash of the key (never store plaintext)
    #[serde(skip)]
    pub key_hash: String,
    /// When the key was created
    pub created_at: u64,
    /// When the key expires (None = never)
    pub expires_at: Option<u64>,
    /// When the key was last used
    pub last_used: u64,
    /// Whether the key is active
    pub active: bool,
}

impl ApiKey {
    /// Check if the key is expired
    pub fn is_expired(&self) -> bool {
        self.expires_at
            .map(|exp| exp < TimeUtils::now_unix_secs())
            .unwrap_or(false)
    }

    /// Check if the key is usable (active and not expired)
    pub fn is_valid(&self) -> bool {
        self.active && !self.is_expired()
    }
}

/// API key store
pub struct ApiKeyStore {
    /// Keys indexed by key ID
    keys: RwLock<HashMap<String, ApiKey>>,
}

impl Default for ApiKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiKeyStore {
    /// Create a new API key store
    pub fn new() -> Self {
        Self {
            keys: RwLock::new(HashMap::new()),
        }
    }

    /// Generate a new API key for a user
    ///
    /// Returns the plaintext key (only returned once) and the key metadata.
    pub fn create_key(
        &self,
        name: String,
        user: &User,
        expires_in_days: Option<u64>,
    ) -> Result<(String, ApiKey)> {
        // Generate random key
        let key_bytes: [u8; 32] = rand::random();
        let key_plaintext = format!(
            "{}{}",
            API_KEY_PREFIX,
            base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, key_bytes)
        );

        // Generate key ID
        let key_id = format!(
            "key_{}",
            &base64::Engine::encode(
                &base64::engine::general_purpose::URL_SAFE_NO_PAD,
                &key_bytes[..8]
            )
        );

        // Hash the key for storage
        let key_hash = password::hash_password(&key_plaintext)?;

        let now = TimeUtils::now_unix_secs();
        let expires_at = expires_in_days.map(|days| now + days * 24 * 60 * 60);

        let api_key = ApiKey {
            id: key_id.clone(),
            name,
            user_email: user.email.clone(),
            role: user.role.clone(),
            key_hash,
            created_at: now,
            expires_at,
            last_used: 0,
            active: true,
        };

        // Store the key
        if let Ok(mut keys) = self.keys.write() {
            keys.insert(key_id, api_key.clone());
        }

        Ok((key_plaintext, api_key))
    }

    /// Validate an API key and return claims if valid
    pub fn validate_key(&self, key_plaintext: &str) -> Result<Claims> {
        // Verify prefix
        if !key_plaintext.starts_with(API_KEY_PREFIX) {
            return Err(Error::generic("Invalid API key format"));
        }

        // Find and validate the key - collect necessary data first
        let matched_key: Option<(String, String, UserRole)> = {
            let keys = self.keys.read_guard()?;

            let mut found = None;
            for api_key in keys.values() {
                if !api_key.is_valid() {
                    continue;
                }

                // Verify the key hash
                if password::verify_password(key_plaintext, &api_key.key_hash)? {
                    found = Some((
                        api_key.id.clone(),
                        api_key.user_email.clone(),
                        api_key.role.clone(),
                    ));
                    break;
                }
            }
            found
        }; // Read lock released here

        // If we found a match, update last_used and return claims
        if let Some((key_id, user_email, role)) = matched_key {
            // Update last used timestamp
            if let Ok(mut keys) = self.keys.write() {
                if let Some(key) = keys.get_mut(&key_id) {
                    key.last_used = TimeUtils::now_unix_secs();
                }
            }

            // Return claims based on API key
            return Ok(Claims::new(
                format!("apikey:{}", key_id),
                user_email,
                role,
                "mcp-context-browser".to_string(),
                86400, // 24 hour validity for API key claims
            ));
        }

        Err(Error::generic("Invalid API key"))
    }

    /// Revoke an API key
    pub fn revoke_key(&self, key_id: &str) -> Result<()> {
        let mut keys = self.keys.write_guard()?;

        if let Some(key) = keys.get_mut(key_id) {
            key.active = false;
            Ok(())
        } else {
            Err(Error::generic("API key not found"))
        }
    }

    /// List all API keys for a user
    pub fn list_keys(&self, user_email: &str) -> Vec<ApiKey> {
        self.keys
            .read()
            .map(|keys| {
                keys.values()
                    .filter(|k| k.user_email == user_email)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Delete an API key permanently
    pub fn delete_key(&self, key_id: &str) -> Result<()> {
        let mut keys = self.keys.write_guard()?;

        if keys.remove(key_id).is_some() {
            Ok(())
        } else {
            Err(Error::generic("API key not found"))
        }
    }
}
