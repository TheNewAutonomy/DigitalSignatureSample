use sha2::{Sha256, Digest};
use std::str;

/// SHA256 hash the string provided.
pub(crate) fn hash_message(message: &str) -> Vec<u8>
{
    // Create a hasher instance
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    hasher.finalize().to_vec()
}

fn to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

#[test]
fn test_hash() {
    let result = hash_message("test");
    let hex_hash = to_hex_string(&result);
    assert!(hex_hash == "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08");
}