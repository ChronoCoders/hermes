use crate::error::{HermesError, Result};
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::Argon2;
use flate2::write::GzEncoder;
use flate2::Compression;
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::io::Write;

const MAGIC_BYTES: &[u8; 4] = b"HRMS";
const VERSION: u8 = 0x01;
const FLAG_COMPRESSED: u8 = 0b00000001;

pub struct EncryptedPackage {
    pub magic: [u8; 4],
    pub version: u8,
    pub flags: u8,
    pub salt: Vec<u8>,
    pub nonce: [u8; 12],
    pub checksum: [u8; 32],
    pub original_size: u64,
    pub filename: Option<String>,
    pub ciphertext: Vec<u8>,
}

impl EncryptedPackage {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.magic);
        bytes.push(self.version);
        bytes.push(self.flags);

        bytes.extend_from_slice(&(self.salt.len() as u16).to_le_bytes());
        bytes.extend_from_slice(&self.salt);

        bytes.extend_from_slice(&self.nonce);
        bytes.extend_from_slice(&self.checksum);
        bytes.extend_from_slice(&self.original_size.to_le_bytes());

        if let Some(ref filename) = self.filename {
            let name_bytes = filename.as_bytes();
            bytes.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
            bytes.extend_from_slice(name_bytes);
        } else {
            bytes.extend_from_slice(&0u16.to_le_bytes());
        }

        bytes.extend_from_slice(&(self.ciphertext.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&self.ciphertext);

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 71 {
            return Err(HermesError::DecryptionFailed);
        }

        let mut pos = 0;

        let mut magic = [0u8; 4];
        magic.copy_from_slice(&bytes[pos..pos + 4]);
        if &magic != MAGIC_BYTES {
            return Err(HermesError::DecryptionFailed);
        }
        pos += 4;

        let version = bytes[pos];
        if version != VERSION {
            return Err(HermesError::DecryptionFailed);
        }
        pos += 1;

        let flags = bytes[pos];
        pos += 1;

        let salt_len = u16::from_le_bytes([bytes[pos], bytes[pos + 1]]) as usize;
        pos += 2;

        let salt = bytes[pos..pos + salt_len].to_vec();
        pos += salt_len;

        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&bytes[pos..pos + 12]);
        pos += 12;

        let mut checksum = [0u8; 32];
        checksum.copy_from_slice(&bytes[pos..pos + 32]);
        pos += 32;

        let original_size = u64::from_le_bytes([
            bytes[pos],
            bytes[pos + 1],
            bytes[pos + 2],
            bytes[pos + 3],
            bytes[pos + 4],
            bytes[pos + 5],
            bytes[pos + 6],
            bytes[pos + 7],
        ]);
        pos += 8;

        let filename_len = u16::from_le_bytes([bytes[pos], bytes[pos + 1]]) as usize;
        pos += 2;

        let filename = if filename_len > 0 {
            let name_bytes = &bytes[pos..pos + filename_len];
            pos += filename_len;
            Some(
                String::from_utf8(name_bytes.to_vec())
                    .map_err(|_| HermesError::DecryptionFailed)?,
            )
        } else {
            None
        };

        let ciphertext_len =
            u32::from_le_bytes([bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]])
                as usize;
        pos += 4;

        let ciphertext = bytes[pos..pos + ciphertext_len].to_vec();

        Ok(EncryptedPackage {
            magic,
            version,
            flags,
            salt,
            nonce,
            checksum,
            original_size,
            filename,
            ciphertext,
        })
    }

    pub fn compressed(&self) -> bool {
        (self.flags & FLAG_COMPRESSED) != 0
    }
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
    let checksum_result = hasher.finalize();
    let mut checksum = [0u8; 32];
    checksum.copy_from_slice(&checksum_result);

    let original_size = plaintext.len() as u64;

    let (data_to_encrypt, flags) = if plaintext.len() > 1024 {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder
            .write_all(plaintext)
            .map_err(|e| HermesError::EncryptionFailed(format!("Compression failed: {}", e)))?;
        let compressed_data = encoder.finish().map_err(|e| {
            HermesError::EncryptionFailed(format!("Compression finish failed: {}", e))
        })?;

        if compressed_data.len() < plaintext.len() {
            (compressed_data, FLAG_COMPRESSED)
        } else {
            (plaintext.to_vec(), 0u8)
        }
    } else {
        (plaintext.to_vec(), 0u8)
    };

    let ciphertext = cipher
        .encrypt(&nonce, data_to_encrypt.as_ref())
        .map_err(|e| HermesError::EncryptionFailed(format!("Encryption failed: {}", e)))?;

    let salt_bytes = salt.as_str().as_bytes().to_vec();

    let package = EncryptedPackage {
        magic: *MAGIC_BYTES,
        version: VERSION,
        flags,
        salt: salt_bytes,
        nonce: nonce_bytes,
        checksum,
        original_size,
        filename,
        ciphertext,
    };

    Ok(package.to_bytes())
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
