use base64::Engine;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

/// SSH-based user identity
/// Uses the user's SSH public key fingerprint as a unique identifier
#[derive(Debug, Clone)]
pub struct SshIdentity {
    /// SHA256 fingerprint of the SSH public key
    pub fingerprint: String,
    /// Short version for display (first 8 chars)
    #[allow(dead_code)]
    pub short_id: String,
}

impl SshIdentity {
    /// Try to get the user's SSH identity from their public key
    pub fn from_ssh_key() -> Option<Self> {
        let home = dirs::home_dir()?;
        
        // Try common SSH key locations in order of preference
        let key_paths = [
            home.join(".ssh/id_ed25519.pub"),
            home.join(".ssh/id_rsa.pub"),
            home.join(".ssh/id_ecdsa.pub"),
        ];

        for path in key_paths {
            if let Some(identity) = Self::from_key_file(&path) {
                return Some(identity);
            }
        }

        None
    }

    /// Create identity from a specific key file
    fn from_key_file(path: &PathBuf) -> Option<Self> {
        let content = fs::read_to_string(path).ok()?;
        
        // SSH public key format: "type base64-key comment"
        // We hash the base64 key part for the fingerprint
        let parts: Vec<&str> = content.trim().split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }

        // Decode the base64 key and hash it (like ssh-keygen -lf does)
        let key_data = base64::engine::general_purpose::STANDARD
            .decode(parts[1])
            .ok()?;

        let hash = Sha256::digest(&key_data);
        let fingerprint = format!("{:x}", hash);
        let short_id = fingerprint[..8].to_string();

        Some(Self {
            fingerprint,
            short_id,
        })
    }

    /// Generate a fallback identity based on machine-specific data
    /// Used when no SSH key is available
    pub fn fallback_identity() -> Self {
        // Use username + home directory as fallback identifier
        let username = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "anonymous".to_string());
        
        // Use home directory path as a machine-specific identifier
        let home_path = dirs::home_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let combined = format!("{}@{}", username, home_path);
        let hash = Sha256::digest(combined.as_bytes());
        let fingerprint = format!("{:x}", hash);
        let short_id = fingerprint[..8].to_string();

        Self {
            fingerprint,
            short_id,
        }
    }

    /// Get identity - tries SSH key first, falls back to machine ID
    pub fn get_or_create() -> Self {
        Self::from_ssh_key().unwrap_or_else(Self::fallback_identity)
    }
}

impl Default for SshIdentity {
    fn default() -> Self {
        Self::get_or_create()
    }
}

