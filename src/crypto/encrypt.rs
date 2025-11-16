use crate::error::{HermesError, Result};
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::Argon2;
use flate2::write::GzEncoder;
use flate2::Compression;
use rsa::rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};
use std::io::Write;

const MAGIC_BYTES: &[u8; 4] = b"HRMS";
const VERSION: u8 = 0x01;
const FLAG_COMPRESSED: u8 = 0b00000001;
const FLAG_MULTI_RECIPIENT: u8 = 0b00000010;

#[derive(Clone)]
pub struct RecipientKey {
    pub name: String,
    pub encrypted_key: Vec<u8>,
}

pub struct EncryptedPackage {
    pub magic: [u8; 4],
    pub version: u8,
    pub flags: u8,
    pub salt: Vec<u8>,
    pub nonce: [u8; 12],
    pub checksum: [u8; 32],
    pub original_size: u64,
    pub expires_at: u64,
    pub filename: Option<String>,
    pub recipients: Vec<RecipientKey>,
    pub ciphertext: Vec<u8>,
}

impl EncryptedPackage {
    #[must_use]
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
        bytes.extend_from_slice(&self.expires_at.to_le_bytes());

        if let Some(ref filename) = self.filename {
            let name_bytes = filename.as_bytes();
            bytes.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
            bytes.extend_from_slice(name_bytes);
        } else {
            bytes.extend_from_slice(&0u16.to_le_bytes());
        }

        bytes.extend_from_slice(&(self.recipients.len() as u16).to_le_bytes());
        for recipient in &self.recipients {
            let name_bytes = recipient.name.as_bytes();
            bytes.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
            bytes.extend_from_slice(name_bytes);
            bytes.extend_from_slice(&(recipient.encrypted_key.len() as u16).to_le_bytes());
            bytes.extend_from_slice(&recipient.encrypted_key);
        }

        bytes.extend_from_slice(&(self.ciphertext.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&self.ciphertext);

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 4 {
            return Err(HermesError::DecryptionFailed);
        }

        let mut magic_check = [0u8; 4];
        magic_check.copy_from_slice(&bytes[0..4]);

        if &magic_check == MAGIC_BYTES {
            Self::from_binary(bytes)
        } else {
            Self::from_json(bytes)
        }
    }

    fn from_binary(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 79 {
            return Err(HermesError::DecryptionFailed);
        }

        let mut pos = 0;

        let mut magic = [0u8; 4];
        magic.copy_from_slice(&bytes[pos..pos + 4]);
        pos += 4;

        let version = bytes[pos];
        pos += 1;

        let flags = bytes[pos];
        pos += 1;

        let salt_len = u16::from_le_bytes([bytes[pos], bytes[pos + 1]]) as usize;
        pos += 2;

        if pos + salt_len + 12 + 32 + 8 + 8 + 2 > bytes.len() {
            return Err(HermesError::DecryptionFailed);
        }

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

        let expires_at = u64::from_le_bytes([
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
            if pos + filename_len > bytes.len() {
                return Err(HermesError::DecryptionFailed);
            }
            let name_bytes = &bytes[pos..pos + filename_len];
            pos += filename_len;
            Some(
                String::from_utf8(name_bytes.to_vec())
                    .map_err(|_| HermesError::DecryptionFailed)?,
            )
        } else {
            None
        };

        if pos + 2 > bytes.len() {
            return Err(HermesError::DecryptionFailed);
        }

        let num_recipients = u16::from_le_bytes([bytes[pos], bytes[pos + 1]]) as usize;
        pos += 2;

        let mut recipients = Vec::new();
        for _ in 0..num_recipients {
            if pos + 2 > bytes.len() {
                return Err(HermesError::DecryptionFailed);
            }

            let name_len = u16::from_le_bytes([bytes[pos], bytes[pos + 1]]) as usize;
            pos += 2;

            if pos + name_len + 2 > bytes.len() {
                return Err(HermesError::DecryptionFailed);
            }

            let name = String::from_utf8(bytes[pos..pos + name_len].to_vec())
                .map_err(|_| HermesError::DecryptionFailed)?;
            pos += name_len;

            let key_len = u16::from_le_bytes([bytes[pos], bytes[pos + 1]]) as usize;
            pos += 2;

            if pos + key_len > bytes.len() {
                return Err(HermesError::DecryptionFailed);
            }

            let encrypted_key = bytes[pos..pos + key_len].to_vec();
            pos += key_len;

            recipients.push(RecipientKey {
                name,
                encrypted_key,
            });
        }

        if pos + 4 > bytes.len() {
            return Err(HermesError::DecryptionFailed);
        }

        let ciphertext_len =
            u32::from_le_bytes([bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]])
                as usize;
        pos += 4;

        if pos + ciphertext_len > bytes.len() {
            return Err(HermesError::DecryptionFailed);
        }

        let ciphertext = bytes[pos..pos + ciphertext_len].to_vec();

        Ok(EncryptedPackage {
            magic,
            version,
            flags,
            salt,
            nonce,
            checksum,
            original_size,
            expires_at,
            filename,
            recipients,
            ciphertext,
        })
    }

    fn from_json(bytes: &[u8]) -> Result<Self> {
        use base64::{engine::general_purpose, Engine as _};
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        struct JsonPackage {
            salt: String,
            nonce: String,
            ciphertext: String,
            #[serde(default)]
            data_type: String,
            #[serde(default)]
            filename: Option<String>,
            #[serde(default)]
            compressed: bool,
            #[serde(default)]
            original_size: u64,
            #[serde(default)]
            checksum: String,
        }

        let json_pkg: JsonPackage =
            serde_json::from_slice(bytes).map_err(|_| HermesError::DecryptionFailed)?;

        let salt = json_pkg.salt.as_bytes().to_vec();

        let nonce_bytes = general_purpose::STANDARD
            .decode(&json_pkg.nonce)
            .map_err(|_| HermesError::DecryptionFailed)?;
        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&nonce_bytes);

        let ciphertext = general_purpose::STANDARD
            .decode(&json_pkg.ciphertext)
            .map_err(|_| HermesError::DecryptionFailed)?;

        let checksum = if json_pkg.checksum.is_empty() {
            [0u8; 32]
        } else {
            let checksum_bytes =
                hex::decode(&json_pkg.checksum).map_err(|_| HermesError::DecryptionFailed)?;
            let mut checksum_arr = [0u8; 32];
            checksum_arr.copy_from_slice(&checksum_bytes);
            checksum_arr
        };

        let flags = if json_pkg.compressed {
            FLAG_COMPRESSED
        } else {
            0
        };

        Ok(EncryptedPackage {
            magic: *MAGIC_BYTES,
            version: VERSION,
            flags,
            salt,
            nonce,
            checksum,
            original_size: json_pkg.original_size,
            expires_at: 0,
            filename: json_pkg.filename,
            recipients: Vec::new(),
            ciphertext,
        })
    }

    #[must_use]
    pub fn compressed(&self) -> bool {
        (self.flags & FLAG_COMPRESSED) != 0
    }

    #[must_use]
    pub fn is_multi_recipient(&self) -> bool {
        (self.flags & FLAG_MULTI_RECIPIENT) != 0
    }

    #[must_use]
    pub fn is_expired(&self) -> bool {
        if self.expires_at == 0 {
            return false;
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now > self.expires_at
    }
}

pub fn encrypt_data(
    plaintext: &[u8],
    password: &str,
    filename: Option<String>,
    ttl_hours: Option<u64>,
) -> Result<Vec<u8>> {
    encrypt_data_multi(plaintext, Some(password), filename, ttl_hours, None)
}

pub fn encrypt_data_multi(
    plaintext: &[u8],
    password: Option<&str>,
    filename: Option<String>,
    ttl_hours: Option<u64>,
    recipient_names: Option<Vec<String>>,
) -> Result<Vec<u8>> {
    let mut data_key = [0u8; 32];
    let mut flags = 0u8;
    let salt;
    let recipients;

    if let Some(names) = recipient_names {
        OsRng.fill_bytes(&mut data_key);

        salt = vec![0u8; 0];
        flags |= FLAG_MULTI_RECIPIENT;

        let recipients_dir = dirs::home_dir()
            .ok_or_else(|| HermesError::ConfigError("Could not find home directory".to_string()))?
            .join(".hermes")
            .join("recipients");

        let mut recipient_list = Vec::new();

        for name in names {
            let pubkey_path = recipients_dir.join(format!("{name}.pub"));
            if !pubkey_path.exists() {
                return Err(HermesError::ConfigError(format!(
                    "Recipient public key not found: {name}"
                )));
            }

            let public_key = crate::crypto::load_public_key(pubkey_path.to_str().unwrap())?;
            let encrypted_key = crate::crypto::encrypt_key_for_recipient(&data_key, &public_key)?;

            recipient_list.push(RecipientKey {
                name,
                encrypted_key,
            });
        }

        recipients = recipient_list;
    } else if let Some(pwd) = password {
        let salt_string = SaltString::generate(OsRng);
        let key = derive_key(pwd, &salt_string)?;
        data_key.copy_from_slice(&key);
        salt = salt_string.as_str().as_bytes().to_vec();
        recipients = Vec::new();
    } else {
        return Err(HermesError::EncryptionFailed(
            "Either password or recipients must be provided".to_string(),
        ));
    }

    let cipher = Aes256Gcm::new_from_slice(&data_key)
        .map_err(|e| HermesError::EncryptionFailed(format!("Cipher creation failed: {e}")))?;

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from(nonce_bytes);

    let mut hasher = Sha256::new();
    hasher.update(plaintext);
    let checksum_result = hasher.finalize();
    let mut checksum = [0u8; 32];
    checksum.copy_from_slice(&checksum_result);

    let original_size = plaintext.len() as u64;

    let (data_to_encrypt, compression_flag) = if plaintext.len() > 1024 {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder
            .write_all(plaintext)
            .map_err(|e| HermesError::EncryptionFailed(format!("Compression failed: {e}")))?;
        let compressed_data = encoder.finish().map_err(|e| {
            HermesError::EncryptionFailed(format!("Compression finish failed: {e}"))
        })?;

        if compressed_data.len() < plaintext.len() {
            (compressed_data, FLAG_COMPRESSED)
        } else {
            (plaintext.to_vec(), 0u8)
        }
    } else {
        (plaintext.to_vec(), 0u8)
    };

    flags |= compression_flag;

    let ciphertext = cipher
        .encrypt(&nonce, data_to_encrypt.as_ref())
        .map_err(|e| HermesError::EncryptionFailed(format!("Encryption failed: {e}")))?;

    let expires_at = if let Some(hours) = ttl_hours {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now + (hours * 3600)
    } else {
        0
    };

    let package = EncryptedPackage {
        magic: *MAGIC_BYTES,
        version: VERSION,
        flags,
        salt,
        nonce: nonce_bytes,
        checksum,
        original_size,
        expires_at,
        filename,
        recipients,
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
