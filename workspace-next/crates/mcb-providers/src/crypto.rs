//! Cryptographic provider trait and types
//!
//! Defines the interface for cryptographic operations used by providers
//! that need encryption capabilities (e.g., EncryptedVectorStoreProvider).
//!
//! ## Architecture
//!
//! This follows Dependency Inversion Principle:
//! - The trait is defined here (mcb-providers)
//! - The implementation lives in mcb-infrastructure (CryptoService)
//! - Providers depend on the abstraction, not the concrete implementation

use mcb_domain::error::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Cryptographic provider trait
///
/// Defines the interface for encryption/decryption operations.
/// Implementations provide the actual cryptographic primitives.
///
/// # Example
///
/// ```ignore
/// use mcb_providers::crypto::CryptoProvider;
///
/// async fn encrypt_metadata(
///     crypto: &dyn CryptoProvider,
///     data: &[u8],
/// ) -> Result<EncryptedData> {
///     crypto.encrypt(data)
/// }
/// ```
pub trait CryptoProvider: Send + Sync {
    /// Encrypt plaintext data
    ///
    /// # Arguments
    ///
    /// * `plaintext` - The data to encrypt
    ///
    /// # Returns
    ///
    /// Encrypted data container with ciphertext and nonce
    fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedData>;

    /// Decrypt encrypted data
    ///
    /// # Arguments
    ///
    /// * `encrypted_data` - The encrypted data container
    ///
    /// # Returns
    ///
    /// The decrypted plaintext
    fn decrypt(&self, encrypted_data: &EncryptedData) -> Result<Vec<u8>>;
}

/// Encrypted data container
///
/// Holds the ciphertext and nonce produced by encryption.
/// Can be serialized for storage in vector store metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedData {
    /// The encrypted ciphertext
    pub ciphertext: Vec<u8>,
    /// The nonce used for encryption
    pub nonce: Vec<u8>,
}

impl EncryptedData {
    /// Create a new encrypted data container
    pub fn new(ciphertext: Vec<u8>, nonce: Vec<u8>) -> Self {
        Self { ciphertext, nonce }
    }
}

impl fmt::Display for EncryptedData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EncryptedData {{ ciphertext: {} bytes, nonce: {} bytes }}",
            self.ciphertext.len(),
            self.nonce.len()
        )
    }
}
