//! Hash Calculator Service - Computes file content hashes
//!
//! Single Responsibility: Calculate cryptographic hashes for files.

use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::Path;

/// Hash calculation service
pub struct HashCalculator;

impl Default for HashCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl HashCalculator {
    /// Create a new hash calculator
    pub fn new() -> Self {
        Self
    }

    /// Calculate SHA-256 hash of file contents
    pub fn hash_file(&self, path: &Path) -> Option<String> {
        let mut file = fs::File::open(path).ok()?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];

        loop {
            let bytes_read = file.read(&mut buffer).ok()?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        Some(format!("{:x}", hasher.finalize()))
    }

    /// Calculate SHA-256 hash of string content
    pub fn hash_content(&self, content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Calculate SHA-256 hash of bytes
    pub fn hash_bytes(&self, bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        format!("{:x}", hasher.finalize())
    }
}
