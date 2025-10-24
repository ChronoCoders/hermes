use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use argon2::Argon2;
use argon2::password_hash::{PasswordHasher, SaltString};
use base64::{Engine as _, engine::general_purpose};
use crate::error::{HermesError, Result};
use crate::crypto::encrypt::EncryptedPackage;
use flate2::read::GzDecoder;
use std::io::Read;
use sha2::{Sha256, Digest};

pub fn decrypt_data(encrypted: &[u8], password: &str) -> Result<Vec<u8>> {
    let package: EncryptedPackage = serde_json::from_slice(encrypted)
        .map_err(|_| HermesError::DecryptionFailed)?;
    
    let salt = SaltString::from_b64(&package.salt)
        .map_err(|_| HermesError::DecryptionFailed)?;
    
    let key = derive_key(password, &salt)?;
    
    let nonce_bytes = general_purpose::STANDARD
        .decode(&package.nonce)
        .map_err(|_| HermesError::DecryptionFailed)?;
    
    let ciphertext = general_purpose::STANDARD
        .decode(&package.ciphertext)
        .map_err(|_| HermesError::DecryptionFailed)?;
    
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|_| HermesError::DecryptionFailed)?;
    
    let nonce_array: [u8; 12] = nonce_bytes.try_into()
        .map_err(|_| HermesError::DecryptionFailed)?;
    let nonce = Nonce::from(nonce_array);
    
    let decrypted = cipher
        .decrypt(&nonce, ciphertext.as_ref())
        .map_err(|_| HermesError::DecryptionFailed)?;
    
    let plaintext = if package.compressed {
        let mut decoder = GzDecoder::new(&decrypted[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)
            .map_err(|_| HermesError::DecryptionFailed)?;
        decompressed
    } else {
        decrypted
    };
    
    let mut hasher = Sha256::new();
    hasher.update(&plaintext);
    let calculated_checksum = format!("{:x}", hasher.finalize());
    
    if calculated_checksum != package.checksum {
        return Err(HermesError::DecryptionFailed);
    }
    
    Ok(plaintext)
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