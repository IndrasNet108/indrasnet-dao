//! Helper functions for idea operations

use sha2::{Sha256, Digest};

/// Normalize idea text
pub(crate) fn normalize_idea_text(text: &str) -> String {
    text
        .trim()                           // Remove leading/trailing whitespace
        .replace("\r\n", "\n")           // Normalize line endings (CRLF → LF)
        .replace('\r', "\n")             // Handle Mac-style line endings
        .to_string()
}

/// Compute idea hash
pub(crate) fn compute_idea_hash(normalized_text: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(normalized_text.as_bytes());
    hasher.finalize().into()
}
