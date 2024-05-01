use ed25519_dalek::{Keypair, Signature, Signer};
use std::fs;

/// Sign the message passed in using the configured private key
pub(crate) fn sign_message(message: &str) -> Result<String, ed25519_dalek::ed25519::Error>
{
    // Load a private key from a file
    let key_file_path = "private_key.dat";
    let key_bytes = fs::read(key_file_path);

    // Create a Keypair from the bytes
    let keypair = Keypair::from_bytes(&key_bytes.unwrap())?;

    // Define a message
    let message: &[u8] = message.as_bytes();

    // Sign the message
    let signature: Signature = keypair.sign(message);

    // Print out the signature in a hexadecimal format
    println!("Signature: {:?}", signature.to_bytes());

    Ok(signature.to_string())
}