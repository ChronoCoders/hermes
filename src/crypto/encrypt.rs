use crate::error::{HermesError, Result};
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::Argon2;
use base64::{engine::general_purpose, Engine as _};
use flate2::write::GzEncoder;
use flate2::Compression;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct EncryptedPackage {
    pub salt: String,
    pub nonce: String,
    pub ciphertext: String,
    pub data_type: String,
    pub filename: Option<String>,
    pub compressed: bool,
    pub original_size: usize,
    pub checksum: String,
}

pub fn encrypt_data(plaintext: &[u8], password: &str, filename: Option<String>) -> Result<Vec<u8>> {
    let salt = SaltString::generate(&mut OsRng);

    let key = derive_key(password, &salt)?;

    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| HermesError::EncryptionFailed(format!("Cipher creation failed: {}", e)))?;

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from(nonce_bytes);

    let mut hasher = Sha256::new();
    hasher.update(plaintext);
    let checksum = format!("{:x}", hasher.finalize());

    let original_size = plaintext.len();
    let (data_to_encrypt, compressed) = if plaintext.len() > 1024 {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder
            .write_all(plaintext)
            .map_err(|e| HermesError::EncryptionFailed(format!("Compression failed: {}", e)))?;
        let compressed_data = encoder.finish().map_err(|e| {
            HermesError::EncryptionFailed(format!("Compression finish failed: {}", e))
        })?;

        if compressed_data.len() < plaintext.len() {
            (compressed_data, true)
        } else {
            (plaintext.to_vec(), false)
        }
    } else {
        (plaintext.to_vec(), false)
    };

    let ciphertext = cipher
        .encrypt(&nonce, data_to_encrypt.as_ref())
        .map_err(|e| HermesError::EncryptionFailed(format!("Encryption failed: {}", e)))?;

    let package = EncryptedPackage {
        salt: salt.to_string(),
        nonce: general_purpose::STANDARD.encode(nonce_bytes),
        ciphertext: general_purpose::STANDARD.encode(&ciphertext),
        data_type: if filename.is_some() {
            "file".to_string()
        } else {
            "text".to_string()
        },
        filename,
        compressed,
        original_size,
        checksum,
    };

    serde_json::to_vec(&package)
        .map_err(|e| HermesError::EncryptionFailed(format!("Serialization failed: {}", e)))
}

fn derive_key(password: &str, salt: &SaltString) -> Result<[u8; 32]> {
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), salt)
        .map_err(|_| HermesError::KeyDerivationFailed)?;

    let hash_bytes = hash.hash.ok_or(HermesError::KeyDerivationFailed)?;
    let bytes = hash_bytes.as_bytes();

    let mut key = [0u8; 32];
    key.copy_from_slice(&bytes[..32]);
    Ok(key)
}
