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
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

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

// ============================================================================
// API Key Persistence (SEC-002)
// ============================================================================

/// Trait for API key persistence (allows different backends)
///
/// Implementations can store API keys in files, databases, or external
/// secret managers while keeping the same interface.
#[async_trait]
pub trait ApiKeyPersistence: Send + Sync {
    /// Load all API keys from persistent storage
    async fn load(&self) -> Result<HashMap<String, ApiKey>>;

    /// Save all API keys to persistent storage
    async fn save(&self, keys: &HashMap<String, ApiKey>) -> Result<()>;
}

/// File-based API key persistence
///
/// Stores API keys in a JSON file with key hashes (not plaintext keys).
/// Suitable for single-server deployments.
pub struct FileApiKeyPersistence {
    path: PathBuf,
}

impl FileApiKeyPersistence {
    /// Create a new file-based persistence backend
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Create with default path in data directory
    pub fn default_path(data_dir: &std::path::Path) -> Self {
        Self::new(data_dir.join("api_keys.json"))
    }
}

/// Serializable version of ApiKey for persistence (includes hash)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredApiKey {
    pub id: String,
    pub name: String,
    pub user_email: String,
    pub role: UserRole,
    pub key_hash: String,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub last_used: u64,
    pub active: bool,
}

impl From<&ApiKey> for StoredApiKey {
    fn from(key: &ApiKey) -> Self {
        Self {
            id: key.id.clone(),
            name: key.name.clone(),
            user_email: key.user_email.clone(),
            role: key.role.clone(),
            key_hash: key.key_hash.clone(),
            created_at: key.created_at,
            expires_at: key.expires_at,
            last_used: key.last_used,
            active: key.active,
        }
    }
}

impl From<StoredApiKey> for ApiKey {
    fn from(stored: StoredApiKey) -> Self {
        Self {
            id: stored.id,
            name: stored.name,
            user_email: stored.user_email,
            role: stored.role,
            key_hash: stored.key_hash,
            created_at: stored.created_at,
            expires_at: stored.expires_at,
            last_used: stored.last_used,
            active: stored.active,
        }
    }
}

#[async_trait]
impl ApiKeyPersistence for FileApiKeyPersistence {
    async fn load(&self) -> Result<HashMap<String, ApiKey>> {
        if !self.path.exists() {
            return Ok(HashMap::new());
        }

        let content = tokio::fs::read_to_string(&self.path)
            .await
            .map_err(|e| Error::io(format!("Failed to read API keys file: {}", e)))?;

        let stored: HashMap<String, StoredApiKey> = serde_json::from_str(&content)
            .map_err(|e| Error::config(format!("Failed to parse API keys file: {}", e)))?;

        Ok(stored.into_iter().map(|(k, v)| (k, v.into())).collect())
    }

    async fn save(&self, keys: &HashMap<String, ApiKey>) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| Error::io(format!("Failed to create API keys directory: {}", e)))?;
        }

        let stored: HashMap<String, StoredApiKey> =
            keys.iter().map(|(k, v)| (k.clone(), v.into())).collect();

        let content = serde_json::to_string_pretty(&stored)
            .map_err(|e| Error::config(format!("Failed to serialize API keys: {}", e)))?;

        // Write atomically using temp file + rename
        let temp_path = self.path.with_extension("json.tmp");
        tokio::fs::write(&temp_path, &content)
            .await
            .map_err(|e| Error::io(format!("Failed to write API keys file: {}", e)))?;

        tokio::fs::rename(&temp_path, &self.path)
            .await
            .map_err(|e| Error::io(format!("Failed to rename API keys file: {}", e)))?;

        Ok(())
    }
}

/// No-op API key persistence (in-memory only, for backward compatibility)
///
/// Keys are lost when the server restarts. Suitable for testing and
/// development but NOT for production deployments.
pub struct NoOpApiKeyPersistence;

#[async_trait]
impl ApiKeyPersistence for NoOpApiKeyPersistence {
    async fn load(&self) -> Result<HashMap<String, ApiKey>> {
        Ok(HashMap::new())
    }

    async fn save(&self, _keys: &HashMap<String, ApiKey>) -> Result<()> {
        Ok(())
    }
}

/// API key store with pluggable persistence backend
pub struct ApiKeyStore {
    /// Keys indexed by key ID
    keys: RwLock<HashMap<String, ApiKey>>,
    /// Persistence backend for durable storage
    persistence: Arc<dyn ApiKeyPersistence>,
}

impl Default for ApiKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiKeyStore {
    /// Create a new API key store with in-memory storage (backward compatible)
    ///
    /// Keys will be lost when the server restarts. Use `with_persistence()` for
    /// durable storage in production.
    pub fn new() -> Self {
        Self {
            keys: RwLock::new(HashMap::new()),
            persistence: Arc::new(NoOpApiKeyPersistence),
        }
    }

    /// Create a new API key store with a custom persistence backend
    pub fn with_persistence(persistence: Arc<dyn ApiKeyPersistence>) -> Self {
        Self {
            keys: RwLock::new(HashMap::new()),
            persistence,
        }
    }

    /// Create a new API key store with file-based persistence
    pub fn with_file_persistence(data_dir: &std::path::Path) -> Self {
        Self::with_persistence(Arc::new(FileApiKeyPersistence::default_path(data_dir)))
    }

    /// Load keys from persistent storage
    ///
    /// Should be called at startup to restore previously stored keys.
    pub async fn load(&self) -> Result<()> {
        let loaded_keys = self.persistence.load().await?;

        let mut keys = self.keys.write_guard()?;
        *keys = loaded_keys;

        Ok(())
    }

    /// Save keys to persistent storage
    async fn save(&self) -> Result<()> {
        let keys = self.keys.read_guard()?.clone();
        self.persistence.save(&keys).await
    }

    /// Generate a new API key for a user
    ///
    /// Returns the plaintext key (only returned once) and the key metadata.
    /// The key is persisted to durable storage if a persistence backend is configured.
    pub async fn create_key(
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
        {
            let mut keys = self.keys.write_guard()?;
            keys.insert(key_id, api_key.clone());
        }

        // Persist to durable storage
        self.save().await?;

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
    ///
    /// The key remains in storage but is marked as inactive.
    /// Changes are persisted to durable storage.
    pub async fn revoke_key(&self, key_id: &str) -> Result<()> {
        {
            let mut keys = self.keys.write_guard()?;

            if let Some(key) = keys.get_mut(key_id) {
                key.active = false;
            } else {
                return Err(Error::generic("API key not found"));
            }
        }

        // Persist to durable storage
        self.save().await?;

        Ok(())
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
    ///
    /// The key is removed from storage completely.
    /// Changes are persisted to durable storage.
    pub async fn delete_key(&self, key_id: &str) -> Result<()> {
        {
            let mut keys = self.keys.write_guard()?;

            if keys.remove(key_id).is_none() {
                return Err(Error::generic("API key not found"));
            }
        }

        // Persist to durable storage
        self.save().await?;

        Ok(())
    }
}
