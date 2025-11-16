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
    pub y: Vec<u8>,
    pub checksum: String,
}

impl Share {
    pub fn verify(&self) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(&[self.id]);
        hasher.update(&[self.threshold]);
        hasher.update(&[self.total_shares]);
        hasher.update(&[self.x]);
        hasher.update(&self.y);
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
        hasher.update(&[self.id]);
        hasher.update(&[self.threshold]);
        hasher.update(&[self.total_shares]);
        hasher.update(&[self.x]);
        hasher.update(&self.y);
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

    for byte_idx in 0..secret.len() {
        let secret_byte = secret[byte_idx];
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
        let points: Vec<(i64, u8)> = shares
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

fn evaluate_polynomial(coefficients: &[BigInt], x: i64, prime: &BigInt) -> u8 {
    let x_big = BigInt::from(x);
    let mut result = BigInt::zero();
    let mut x_power = BigInt::one();

    for coeff in coefficients {
        result = (result + coeff * &x_power) % prime;
        x_power = (x_power * &x_big) % prime;
    }

    result.to_u32_digits().1.first().copied().unwrap_or(0) as u8
}

fn lagrange_interpolation(points: &[(i64, u8)], prime: &BigInt) -> u8 {
    let mut result = BigInt::zero();

    for i in 0..points.len() {
        let (xi, yi) = points[i];
        let mut numerator = BigInt::one();
        let mut denominator = BigInt::one();

        for j in 0..points.len() {
            if i != j {
                let (xj, _) = points[j];
                numerator = (numerator * BigInt::from(-xj)) % prime;
                denominator = (denominator * BigInt::from(xi - xj)) % prime;
            }
        }

        if denominator.sign() == Sign::Minus {
            denominator = prime + denominator;
        }

        let inv_denominator = mod_inverse(&denominator, prime);
        let term = (BigInt::from(yi) * numerator * inv_denominator) % prime;
        result = (result + term) % prime;
    }

    if result.sign() == Sign::Minus {
        result = prime + result;
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
        t = t + m;
    }

    t
}
