use sha256::digest;

/// SHA256 hash the string provided.
pub(crate) fn hash_message(message: &str) -> String
{
    let hash = digest(message);
    println!("Hash: {}", hash);
    hash
}

#[test]
fn test_hash() {
    let result = hash_message("test");
    assert!(result == "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08");
}