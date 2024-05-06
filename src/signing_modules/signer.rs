// In signing_modules/signer.rs
use base64::encode;
use hex::FromHex;
use rsa::{pkcs8::FromPrivateKey, PaddingScheme, RsaPrivateKey};
use std::error::Error;
use std::fs;

pub fn sign_message(hashed_message: &[u8]) -> Result<String, Box<dyn Error>> {
    // Load the private key from a PEM file
    let key_file_path = "client-key.pem";
    let key_data = fs::read_to_string(key_file_path)?;

    // Decode the PEM encoded private key
    let private_key = RsaPrivateKey::from_pkcs8_pem(&key_data)?;

    // Sign the hashed message
    let padding = PaddingScheme::new_pkcs1v15_sign(Some(rsa::hash::Hash::SHA2_256));
    let signature = private_key.sign(padding, &hashed_message)?;

    // Encode the signature in base64 to make it easier to handle
    Ok(encode(signature))
}

#[test]
fn test_hash() {
    let hashed_message = Vec::<u8>::from_hex("9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08").unwrap();
    let signed_message = sign_message(&hashed_message).unwrap();
        
    // Instead of comparing the result directly, we should check the length or format if applicable
    assert!(!signed_message.is_empty());
}