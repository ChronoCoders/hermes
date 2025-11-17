use crate::error::{HermesError, Result};
use base64::Engine;
use pqcrypto_dilithium::dilithium5;
use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage};
use std::fs;
use std::path::Path;

pub struct DilithiumPublicKey(pub dilithium5::PublicKey);
pub struct DilithiumSecretKey(pub dilithium5::SecretKey);

/// Generate a Dilithium-5 keypair for digital signatures
pub fn generate_dilithium_keypair() -> Result<(DilithiumPublicKey, DilithiumSecretKey)> {
    let (pk, sk) = dilithium5::keypair();
    Ok((DilithiumPublicKey(pk), DilithiumSecretKey(sk)))
}

/// Sign a message with Dilithium
pub fn sign_message(message: &[u8], secret_key: &DilithiumSecretKey) -> Vec<u8> {
    let signed = dilithium5::sign(message, &secret_key.0);
    signed.as_bytes().to_vec()
}

/// Verify and extract message from signed data
pub fn verify_signature(signed_message: &[u8], public_key: &DilithiumPublicKey) -> Result<Vec<u8>> {
    let sm = dilithium5::SignedMessage::from_bytes(signed_message)
        .map_err(|_| HermesError::DecryptionFailed)?;

    let verified = dilithium5::open(&sm, &public_key.0)
        .map_err(|_| HermesError::DecryptionFailed)?;

    Ok(verified)
}

/// Save Dilithium public key to file
pub fn save_dilithium_public_key(public_key: &DilithiumPublicKey, path: &Path) -> Result<()> {
    let bytes = public_key.0.as_bytes();
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
    let pem = format!(
        "-----BEGIN DILITHIUM PUBLIC KEY-----\n{}\n-----END DILITHIUM PUBLIC KEY-----\n",
        encoded
            .chars()
            .collect::<Vec<_>>()
            .chunks(64)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );

    fs::write(path, pem).map_err(|e| {
        HermesError::KeyGenerationFailed(format!("Failed to save Dilithium public key: {e}"))
    })?;

    Ok(())
}

/// Save Dilithium secret key to file
pub fn save_dilithium_secret_key(secret_key: &DilithiumSecretKey, path: &Path) -> Result<()> {
    let bytes = secret_key.0.as_bytes();
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
    let pem = format!(
        "-----BEGIN DILITHIUM PRIVATE KEY-----\n{}\n-----END DILITHIUM PRIVATE KEY-----\n",
        encoded
            .chars()
            .collect::<Vec<_>>()
            .chunks(64)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );

    fs::write(path, pem).map_err(|e| {
        HermesError::KeyGenerationFailed(format!("Failed to save Dilithium secret key: {e}"))
    })?;

    Ok(())
}

/// Load Dilithium public key from file
pub fn load_dilithium_public_key(path: &str) -> Result<DilithiumPublicKey> {
    let content = fs::read_to_string(path)
        .map_err(|e| HermesError::ConfigError(format!("Failed to read Dilithium public key: {e}")))?;

    let encoded = content
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<String>();

    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .map_err(|e| HermesError::ConfigError(format!("Failed to decode Dilithium public key: {e}")))?;

    let key = dilithium5::PublicKey::from_bytes(&bytes)
        .map_err(|_| HermesError::ConfigError("Invalid Dilithium public key".to_string()))?;

    Ok(DilithiumPublicKey(key))
}

/// Load Dilithium secret key from file
pub fn load_dilithium_secret_key(path: &str) -> Result<DilithiumSecretKey> {
    let content = fs::read_to_string(path)
        .map_err(|e| HermesError::ConfigError(format!("Failed to read Dilithium secret key: {e}")))?;

    let encoded = content
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<String>();

    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .map_err(|e| HermesError::ConfigError(format!("Failed to decode Dilithium secret key: {e}")))?;

    let key = dilithium5::SecretKey::from_bytes(&bytes)
        .map_err(|_| HermesError::ConfigError("Invalid Dilithium secret key".to_string()))?;

    Ok(DilithiumSecretKey(key))
}

/// Get fingerprint of Dilithium public key
pub fn get_dilithium_fingerprint(public_key: &DilithiumPublicKey) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(public_key.0.as_bytes());
    let hash = hasher.finalize();
    hex::encode(&hash[..8])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dilithium_sign_verify() {
        let (pk, sk) = generate_dilithium_keypair().unwrap();
        let message = b"Test message for signing";

        let signed = sign_message(message, &sk);
        let verified = verify_signature(&signed, &pk).unwrap();

        assert_eq!(verified, message);
    }

    #[test]
    fn test_dilithium_wrong_key() {
        let (_pk1, sk1) = generate_dilithium_keypair().unwrap();
        let (pk2, _sk2) = generate_dilithium_keypair().unwrap();
        let message = b"Test message";

        let signed = sign_message(message, &sk1);
        let result = verify_signature(&signed, &pk2);

        assert!(result.is_err());
    }

    #[test]
    fn test_dilithium_fingerprint() {
        let (pk, _sk) = generate_dilithium_keypair().unwrap();
        let fingerprint = get_dilithium_fingerprint(&pk);
        assert_eq!(fingerprint.len(), 16);
    }
}
