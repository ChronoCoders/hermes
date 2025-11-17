use crate::error::{HermesError, Result};
use base64::Engine;
use pqc_kyber::{decapsulate, encapsulate, keypair, KYBER_CIPHERTEXTBYTES, KYBER_PUBLICKEYBYTES, KYBER_SECRETKEYBYTES};
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;

pub struct KyberPublicKey(pub [u8; KYBER_PUBLICKEYBYTES]);
pub struct KyberSecretKey(pub [u8; KYBER_SECRETKEYBYTES]);

/// Generate a Kyber-1024 keypair
pub fn generate_kyber_keypair() -> Result<(KyberPublicKey, KyberSecretKey)> {
    let keys = keypair(&mut OsRng)
        .map_err(|e| HermesError::KeyGenerationFailed(format!("Kyber keypair generation failed: {e:?}")))?;

    Ok((KyberPublicKey(keys.public), KyberSecretKey(keys.secret)))
}

/// Encrypt a 32-byte AES key using Kyber KEM
/// Returns the ciphertext and the encapsulated shared secret
pub fn encrypt_with_kyber(data_key: &[u8; 32], public_key: &KyberPublicKey) -> Result<Vec<u8>> {
    let (ciphertext, shared_secret) = encapsulate(&public_key.0, &mut OsRng)
        .map_err(|e| HermesError::EncryptionFailed(format!("Kyber encapsulation failed: {e:?}")))?;

    // XOR the data key with the shared secret to encrypt it
    let mut encrypted_key = [0u8; 32];
    for i in 0..32 {
        encrypted_key[i] = data_key[i] ^ shared_secret[i];
    }

    // Return ciphertext + encrypted key
    let mut result = Vec::with_capacity(KYBER_CIPHERTEXTBYTES + 32);
    result.extend_from_slice(&ciphertext);
    result.extend_from_slice(&encrypted_key);

    Ok(result)
}

/// Decrypt the AES key using Kyber KEM
pub fn decrypt_with_kyber(encrypted_data: &[u8], secret_key: &KyberSecretKey) -> Result<[u8; 32]> {
    if encrypted_data.len() != KYBER_CIPHERTEXTBYTES + 32 {
        return Err(HermesError::DecryptionFailed);
    }

    let ciphertext: [u8; KYBER_CIPHERTEXTBYTES] = encrypted_data[..KYBER_CIPHERTEXTBYTES]
        .try_into()
        .map_err(|_| HermesError::DecryptionFailed)?;

    let encrypted_key = &encrypted_data[KYBER_CIPHERTEXTBYTES..];

    let shared_secret = decapsulate(&ciphertext, &secret_key.0)
        .map_err(|_| HermesError::DecryptionFailed)?;

    // XOR to recover the original key
    let mut data_key = [0u8; 32];
    for i in 0..32 {
        data_key[i] = encrypted_key[i] ^ shared_secret[i];
    }

    Ok(data_key)
}

/// Save Kyber public key to file
pub fn save_kyber_public_key(public_key: &KyberPublicKey, path: &Path) -> Result<()> {
    let encoded = base64::engine::general_purpose::STANDARD.encode(public_key.0);
    let pem = format!(
        "-----BEGIN KYBER PUBLIC KEY-----\n{}\n-----END KYBER PUBLIC KEY-----\n",
        encoded.chars().collect::<Vec<_>>().chunks(64).map(|c| c.iter().collect::<String>()).collect::<Vec<_>>().join("\n")
    );

    fs::write(path, pem)
        .map_err(|e| HermesError::KeyGenerationFailed(format!("Failed to save Kyber public key: {e}")))?;

    Ok(())
}

/// Save Kyber secret key to file
pub fn save_kyber_secret_key(secret_key: &KyberSecretKey, path: &Path) -> Result<()> {
    let encoded = base64::engine::general_purpose::STANDARD.encode(secret_key.0);
    let pem = format!(
        "-----BEGIN KYBER PRIVATE KEY-----\n{}\n-----END KYBER PRIVATE KEY-----\n",
        encoded.chars().collect::<Vec<_>>().chunks(64).map(|c| c.iter().collect::<String>()).collect::<Vec<_>>().join("\n")
    );

    fs::write(path, pem)
        .map_err(|e| HermesError::KeyGenerationFailed(format!("Failed to save Kyber secret key: {e}")))?;

    Ok(())
}

/// Load Kyber public key from file
pub fn load_kyber_public_key(path: &str) -> Result<KyberPublicKey> {
    let content = fs::read_to_string(path)
        .map_err(|e| HermesError::ConfigError(format!("Failed to read Kyber public key: {e}")))?;

    let encoded = content
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<String>();

    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .map_err(|e| HermesError::ConfigError(format!("Failed to decode Kyber public key: {e}")))?;

    if bytes.len() != KYBER_PUBLICKEYBYTES {
        return Err(HermesError::ConfigError(format!(
            "Invalid Kyber public key size: expected {}, got {}",
            KYBER_PUBLICKEYBYTES,
            bytes.len()
        )));
    }

    let mut key = [0u8; KYBER_PUBLICKEYBYTES];
    key.copy_from_slice(&bytes);

    Ok(KyberPublicKey(key))
}

/// Load Kyber secret key from file
pub fn load_kyber_secret_key(path: &str) -> Result<KyberSecretKey> {
    let content = fs::read_to_string(path)
        .map_err(|e| HermesError::ConfigError(format!("Failed to read Kyber secret key: {e}")))?;

    let encoded = content
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<String>();

    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .map_err(|e| HermesError::ConfigError(format!("Failed to decode Kyber secret key: {e}")))?;

    if bytes.len() != KYBER_SECRETKEYBYTES {
        return Err(HermesError::ConfigError(format!(
            "Invalid Kyber secret key size: expected {}, got {}",
            KYBER_SECRETKEYBYTES,
            bytes.len()
        )));
    }

    let mut key = [0u8; KYBER_SECRETKEYBYTES];
    key.copy_from_slice(&bytes);

    Ok(KyberSecretKey(key))
}

/// Get fingerprint of Kyber public key
pub fn get_kyber_fingerprint(public_key: &KyberPublicKey) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(public_key.0);
    let hash = hasher.finalize();
    hex::encode(&hash[..8])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyber_encrypt_decrypt() {
        let (pk, sk) = generate_kyber_keypair().unwrap();

        let original_key = [42u8; 32];
        let encrypted = encrypt_with_kyber(&original_key, &pk).unwrap();
        let decrypted = decrypt_with_kyber(&encrypted, &sk).unwrap();

        assert_eq!(original_key, decrypted);
    }

    #[test]
    fn test_kyber_different_keys() {
        let (pk1, _sk1) = generate_kyber_keypair().unwrap();
        let (_pk2, sk2) = generate_kyber_keypair().unwrap();

        let original_key = [42u8; 32];
        let encrypted = encrypt_with_kyber(&original_key, &pk1).unwrap();

        // Decrypting with wrong key should fail
        let result = decrypt_with_kyber(&encrypted, &sk2);
        // The decryption won't error but will produce wrong key
        if let Ok(decrypted) = result {
            assert_ne!(original_key, decrypted);
        }
    }

    #[test]
    fn test_kyber_fingerprint() {
        let (pk, _sk) = generate_kyber_keypair().unwrap();
        let fingerprint = get_kyber_fingerprint(&pk);

        assert_eq!(fingerprint.len(), 16); // 8 bytes = 16 hex chars
    }
}
