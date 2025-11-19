use crate::error::{HermesError, Result};
use num_bigint::{BigInt, Sign};
use num_traits::{One, Zero};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Share {
    pub id: u8,
    pub threshold: u8,
    pub total_shares: u8,
    pub x: u8,
    pub y: Vec<u16>,  // u16 to hold values 0-256 (for prime 257)
    pub checksum: String,
}

impl Share {
    pub fn verify(&self) -> bool {
        let mut hasher = Sha256::new();
        hasher.update([self.id]);
        hasher.update([self.threshold]);
        hasher.update([self.total_shares]);
        hasher.update([self.x]);
        // Convert y values to bytes for hashing
        for val in &self.y {
            hasher.update(val.to_le_bytes());
        }
        let calculated = format!("{:x}", hasher.finalize());
        calculated == self.checksum
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(HermesError::SerializationError)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(HermesError::SerializationError)
    }

    pub fn calculate_checksum(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update([self.id]);
        hasher.update([self.threshold]);
        hasher.update([self.total_shares]);
        hasher.update([self.x]);
        // Convert y values to bytes for hashing
        for val in &self.y {
            hasher.update(val.to_le_bytes());
        }
        self.checksum = format!("{:x}", hasher.finalize());
    }
}

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

    let prime = generate_prime();
    let mut shares = Vec::new();

    for (byte_idx, &secret_byte) in secret.iter().enumerate() {
        let coefficients = generate_coefficients(secret_byte, threshold);

        for share_id in 1..=total_shares {
            let x = share_id;
            let y = evaluate_polynomial(&coefficients, x as i64, &prime);

            if byte_idx == 0 {
                shares.push(Share {
                    id: share_id,
                    threshold,
                    total_shares,
                    x,
                    y: vec![y],
                    checksum: String::new(),
                });
            } else {
                shares[(share_id - 1) as usize].y.push(y);
            }
        }
    }

    for share in &mut shares {
        share.calculate_checksum();
    }

    Ok(shares)
}

pub fn recover_secret(shares: &[Share]) -> Result<Vec<u8>> {
    if shares.is_empty() {
        return Err(HermesError::ConfigError("No shares provided".to_string()));
    }

    for share in shares {
        if !share.verify() {
            return Err(HermesError::ConfigError(format!(
                "Share {} failed verification",
                share.id
            )));
        }
    }

    let threshold = shares[0].threshold;
    if (shares.len() as u8) < threshold {
        return Err(HermesError::ConfigError(format!(
            "Need at least {} shares, only {} provided",
            threshold,
            shares.len()
        )));
    }

    let secret_len = shares[0].y.len();
    for share in shares {
        if share.y.len() != secret_len {
            return Err(HermesError::ConfigError(
                "All shares must have same length".to_string(),
            ));
        }
        if share.threshold != threshold {
            return Err(HermesError::ConfigError(
                "All shares must have same threshold".to_string(),
            ));
        }
    }

    let prime = generate_prime();
    let mut secret = Vec::new();

    for byte_idx in 0..secret_len {
        let points: Vec<(i64, u16)> = shares
            .iter()
            .take(threshold as usize)
            .map(|s| (s.x as i64, s.y[byte_idx]))
            .collect();

        let recovered_byte = lagrange_interpolation(&points, &prime);
        secret.push(recovered_byte);
    }

    Ok(secret)
}

fn generate_prime() -> BigInt {
    // Use 257, smallest prime > 256
    // This ensures all byte values (0-255) can be represented
    BigInt::from(257)
}

fn generate_coefficients(secret: u8, threshold: u8) -> Vec<BigInt> {
    let mut coefficients = vec![BigInt::from(secret)];
    let mut rng = rand::thread_rng();

    for _ in 1..threshold {
        let coeff = rng.gen_range(1..256);
        coefficients.push(BigInt::from(coeff));
    }

    coefficients
}

fn evaluate_polynomial(coefficients: &[BigInt], x: i64, prime: &BigInt) -> u16 {
    let x_big = BigInt::from(x);
    let mut result = BigInt::zero();
    let mut x_power = BigInt::one();

    for coeff in coefficients {
        result = (result + coeff * &x_power) % prime;
        x_power = (x_power * &x_big) % prime;
    }

    // Ensure result is positive
    if result.sign() == Sign::Minus {
        result += prime;
    }

    // Convert to u16 - result should be in range [0, 256]
    let (_, digits) = result.to_u32_digits();
    digits.first().copied().unwrap_or(0) as u16
}

fn lagrange_interpolation(points: &[(i64, u16)], prime: &BigInt) -> u8 {
    let mut result = BigInt::zero();

    for (i, &(xi, yi)) in points.iter().enumerate() {
        let mut numerator = BigInt::one();
        let mut denominator = BigInt::one();

        for (j, &(xj, _)) in points.iter().enumerate() {
            if i != j {
                let neg_xj = BigInt::from(-xj);
                numerator = (numerator * neg_xj) % prime;
                if numerator.sign() == Sign::Minus {
                    numerator += prime;
                }

                let diff = BigInt::from(xi - xj);
                denominator = (denominator * diff) % prime;
                if denominator.sign() == Sign::Minus {
                    denominator += prime;
                }
            }
        }

        let inv_denominator = mod_inverse(&denominator, prime);
        let term = (BigInt::from(yi) * numerator % prime * inv_denominator) % prime;
        result = (result + term) % prime;
    }

    if result.sign() == Sign::Minus {
        result += prime;
    }

    result.to_u32_digits().1.first().copied().unwrap_or(0) as u8
}

fn mod_inverse(a: &BigInt, m: &BigInt) -> BigInt {
    let (mut t, mut new_t) = (BigInt::zero(), BigInt::one());
    let (mut r, mut new_r) = (m.clone(), a.clone());

    while !new_r.is_zero() {
        let quotient = &r / &new_r;
        (t, new_t) = (new_t.clone(), t - &quotient * &new_t);
        (r, new_r) = (new_r.clone(), r - quotient * new_r);
    }

    if t < BigInt::zero() {
        t += m;
    }

    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_split_recover() {
        let secret = vec![1, 2, 3, 4, 5];
        let shares = split_secret(&secret, 3, 5).unwrap();
        
        // Use shares 1, 3, 5
        let selected = vec![shares[0].clone(), shares[2].clone(), shares[4].clone()];
        let recovered = recover_secret(&selected).unwrap();
        
        assert_eq!(secret, recovered);
    }
}

    #[test]
    fn test_rsa_key_split_recover() {
        use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
        use rsa::RsaPrivateKey;
        
        // Generate a small test key
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 512).unwrap();
        
        // Get DER bytes
        let key_bytes = private_key.to_pkcs8_der().unwrap().as_bytes().to_vec();
        println!("Original key size: {} bytes", key_bytes.len());
        
        // Split into shares
        let shares = split_secret(&key_bytes, 3, 5).unwrap();
        
        // Recover using shares 1, 3, 5
        let selected = vec![shares[0].clone(), shares[2].clone(), shares[4].clone()];
        let recovered = recover_secret(&selected).unwrap();
        
        assert_eq!(key_bytes, recovered, "Recovered bytes don't match original");
        
        // Try to parse as RSA key
        let recovered_key = RsaPrivateKey::from_pkcs8_der(&recovered).unwrap();
        assert_eq!(private_key, recovered_key);
    }

    #[test]
    fn test_all_byte_values() {
        // Test with all possible byte values
        let secret: Vec<u8> = (0u8..=255).collect();
        let shares = split_secret(&secret, 3, 5).unwrap();
        
        let selected = vec![shares[0].clone(), shares[2].clone(), shares[4].clone()];
        let recovered = recover_secret(&selected).unwrap();
        
        for (i, (orig, rec)) in secret.iter().zip(recovered.iter()).enumerate() {
            if orig != rec {
                println!("Mismatch at index {}: original={}, recovered={}", i, orig, rec);
            }
        }
        
        assert_eq!(secret, recovered);
    }
