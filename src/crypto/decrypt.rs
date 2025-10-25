use crate::crypto::encrypt::EncryptedPackage;
use crate::error::{HermesError, Result};
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::Argon2;
use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};
use std::io::Read;

pub fn decrypt_data(encrypted: &[u8], password: &str) -> Result<Vec<u8>> {
    let package = EncryptedPackage::from_bytes(encrypted)?;

    let salt_str =
        String::from_utf8(package.salt.clone()).map_err(|_| HermesError::DecryptionFailed)?;
    let salt = SaltString::from_b64(&salt_str).map_err(|_| HermesError::DecryptionFailed)?;

    let key = derive_key(password, &salt)?;

    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| HermesError::DecryptionFailed)?;

    let nonce = Nonce::from(package.nonce);

    let decrypted = cipher
        .decrypt(&nonce, package.ciphertext.as_ref())
        .map_err(|_| HermesError::DecryptionFailed)?;

    let plaintext = if package.compressed() {
        let mut decoder = GzDecoder::new(&decrypted[..]);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|_| HermesError::DecryptionFailed)?;
        decompressed
    } else {
        decrypted
    };

    let mut hasher = Sha256::new();
    hasher.update(&plaintext);
    let calculated_checksum = hasher.finalize();

    let mut checksum_array = [0u8; 32];
    checksum_array.copy_from_slice(&calculated_checksum);

    if checksum_array != package.checksum {
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
