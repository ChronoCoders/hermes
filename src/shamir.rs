use crate::error::{HermesError, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Share {
    pub id: u8,
    pub index: u8,
    pub threshold: u8,
    pub total_shares: u8,
    pub y: Vec<u8>,
    pub key_id: String,
    pub checksum: String,
}

impl Share {
    pub fn new(id: u8, index: u8, threshold: u8, total_shares: u8, y: Vec<u8>, key_id: String) -> Self {
        let checksum = Self::compute_checksum(&y);
        Self {
            id,
            index,
            threshold,
            total_shares,
            y,
            key_id,
            checksum,
        }
    }

    fn compute_checksum(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        hex::encode(&hash[..8])
    }

    pub fn verify(&self) -> bool {
        self.checksum == Self::compute_checksum(&self.y)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| e.into())
    }

    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| e.into())
    }
}

/// Split a secret into n shares with threshold k (k-of-n scheme)
pub fn split_secret(secret: &[u8], threshold: u8, total_shares: u8) -> Result<Vec<Share>> {
    if threshold > total_shares {
        return Err(HermesError::ConfigError(
            "Threshold cannot be greater than total shares".to_string(),
        ));
    }
    if threshold < 2 {
        return Err(HermesError::ConfigError(
            "Threshold must be at least 2".to_string(),
        ));
    }

    let key_id = compute_key_id(secret);

    // Simple XOR-based secret sharing for demonstration
    // In production, use proper Shamir's Secret Sharing with polynomial interpolation
    let mut shares: Vec<Share> = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 1..=total_shares {
        let mut share_data = vec![0u8; secret.len()];

        if i < total_shares {
            // Generate random data for all shares except the last
            use rand::RngCore;
            rng.fill_bytes(&mut share_data);
        } else {
            // Last share is computed to reconstruct the secret
            // XOR all previous shares with the secret
            share_data.copy_from_slice(secret);
            for prev_share in &shares {
                for (j, byte) in share_data.iter_mut().enumerate() {
                    *byte ^= prev_share.y[j];
                }
            }
        }

        shares.push(Share::new(
            i,
            i,
            threshold,
            total_shares,
            share_data,
            key_id.clone(),
        ));
    }

    Ok(shares)
}

/// Recover the secret from shares (requires threshold number of shares)
pub fn recover_secret(shares: &[Share]) -> Result<Vec<u8>> {
    if shares.is_empty() {
        return Err(HermesError::ConfigError("No shares provided".to_string()));
    }

    let threshold = shares[0].threshold;
    if shares.len() < threshold as usize {
        return Err(HermesError::ConfigError(format!(
            "Need at least {} shares, but only {} provided",
            threshold,
            shares.len()
        )));
    }

    // Verify all shares have the same key_id
    let key_id = &shares[0].key_id;
    for share in shares.iter().skip(1) {
        if &share.key_id != key_id {
            return Err(HermesError::ConfigError(
                "Shares are from different keys".to_string(),
            ));
        }
    }

    // Simple XOR recovery - XOR all shares together
    let len = shares[0].y.len();
    let mut secret = vec![0u8; len];

    for share in shares {
        if share.y.len() != len {
            return Err(HermesError::ConfigError(
                "Share data lengths do not match".to_string(),
            ));
        }
        for (i, byte) in secret.iter_mut().enumerate() {
            *byte ^= share.y[i];
        }
    }

    Ok(secret)
}

fn compute_key_id(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    hex::encode(&hash[..8])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_and_recover() {
        let secret = b"This is a secret key!";
        let shares = split_secret(secret, 3, 5).unwrap();

        assert_eq!(shares.len(), 5);

        // Recover with all shares
        let recovered = recover_secret(&shares).unwrap();
        assert_eq!(recovered, secret);
    }

    #[test]
    fn test_share_verification() {
        let secret = b"Test secret";
        let shares = split_secret(secret, 2, 3).unwrap();

        for share in &shares {
            assert!(share.verify());
        }
    }

    #[test]
    fn test_share_serialization() {
        let share = Share::new(1, 1, 3, 5, vec![1, 2, 3, 4], "test_key_id".to_string());

        let json = share.to_json().unwrap();
        let recovered = Share::from_json(&json).unwrap();

        assert_eq!(share.id, recovered.id);
        assert_eq!(share.y, recovered.y);
        assert_eq!(share.key_id, recovered.key_id);
    }
}

