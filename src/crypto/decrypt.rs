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

    let key = if package.is_multi_recipient() {
        return Err(HermesError::DecryptionFailed);
    } else {
        let salt_str =
            String::from_utf8(package.salt.clone()).map_err(|_| HermesError::DecryptionFailed)?;
        let salt = SaltString::from_b64(&salt_str).map_err(|_| HermesError::DecryptionFailed)?;

        derive_key(password, &salt)?
    };

    decrypt_with_key(encrypted, &key)
}

pub fn decrypt_data_multi(encrypted: &[u8], recipient_name: &str) -> Result<Vec<u8>> {
    let package = EncryptedPackage::from_bytes(encrypted)?;

    if !package.is_multi_recipient() {
        return Err(HermesError::DecryptionFailed);
    }

    let key_dir = dirs::home_dir()
        .ok_or_else(|| HermesError::ConfigError("Could not find home directory".to_string()))?
        .join(".hermes")
        .join("keys");

    let private_key_path = key_dir.join(format!("{}.pem", recipient_name));
    if !private_key_path.exists() {
        return Err(HermesError::ConfigError(format!(
            "Private key not found for: {}",
            recipient_name
        )));
    }

    let private_key = crate::crypto::load_private_key(private_key_path.to_str().unwrap())?;

    let recipient = package
        .recipients
        .iter()
        .find(|r| r.name == recipient_name)
        .ok_or(HermesError::DecryptionFailed)?;

    let data_key_bytes =
        crate::crypto::decrypt_key_with_private(&recipient.encrypted_key, &private_key)?;

    let mut data_key = [0u8; 32];
    data_key.copy_from_slice(&data_key_bytes);

    decrypt_with_key(encrypted, &data_key)
}

fn decrypt_with_key(encrypted: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
    let package = EncryptedPackage::from_bytes(encrypted)?;

    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| HermesError::DecryptionFailed)?;

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
