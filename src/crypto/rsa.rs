use crate::error::{HermesError, Result};
use rsa::pkcs8::{
    DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding,
};
use rsa::rand_core::OsRng;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::fs;

const RSA_KEY_SIZE: usize = 4096;

pub fn generate_keypair(private_key_path: &str, public_key_path: &str) -> Result<()> {
    let mut rng = OsRng;

    let private_key = RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE)
        .map_err(|e| HermesError::EncryptionFailed(format!("RSA key generation failed: {e}")))?;

    let public_key = RsaPublicKey::from(&private_key);

    let private_pem = private_key
        .to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| HermesError::EncryptionFailed(format!("Private key encoding failed: {e}")))?;

    let public_pem = public_key
        .to_public_key_pem(LineEnding::LF)
        .map_err(|e| HermesError::EncryptionFailed(format!("Public key encoding failed: {e}")))?;

    fs::write(private_key_path, private_pem.as_bytes())?;
    fs::write(public_key_path, public_pem.as_bytes())?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(private_key_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(private_key_path, perms)?;
    }

    Ok(())
}

pub fn load_private_key(path: &str) -> Result<RsaPrivateKey> {
    let pem = fs::read_to_string(path)?;
    RsaPrivateKey::from_pkcs8_pem(&pem).map_err(|_e| HermesError::DecryptionFailed)
}

pub fn load_public_key(path: &str) -> Result<RsaPublicKey> {
    let pem = fs::read_to_string(path)?;
    RsaPublicKey::from_public_key_pem(&pem).map_err(|_e| HermesError::DecryptionFailed)
}

pub fn save_private_key(key: &RsaPrivateKey, path: &str) -> Result<()> {
    let pem = key.to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| HermesError::EncryptionFailed(format!("Private key encoding failed: {e}")))?;
    
    fs::write(path, pem.as_bytes())?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(path, perms)?;
    }
    
    Ok(())
}

pub fn encrypt_key_for_recipient(key: &[u8], public_key: &RsaPublicKey) -> Result<Vec<u8>> {
    let mut rng = OsRng;
    public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, key)
        .map_err(|e| HermesError::EncryptionFailed(format!("RSA encryption failed: {e}")))
}

pub fn decrypt_key_with_private(
    encrypted_key: &[u8],
    private_key: &RsaPrivateKey,
) -> Result<Vec<u8>> {
    private_key
        .decrypt(Pkcs1v15Encrypt, encrypted_key)
        .map_err(|_e| HermesError::DecryptionFailed)
}

pub fn get_key_fingerprint(public_key: &RsaPublicKey) -> Result<String> {
    use sha2::{Digest, Sha256};

    let pem = public_key
        .to_public_key_pem(LineEnding::LF)
        .map_err(|e| HermesError::EncryptionFailed(format!("Public key encoding failed: {e}")))?;

    let mut hasher = Sha256::new();
    hasher.update(pem.as_bytes());
    let hash = hasher.finalize();

    Ok(hex::encode(&hash[..8]))
}
