/*
    Appellation: h256 <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

use super::{H256Hash, Hashable};
use rand::Rng;
use ring::digest::digest;
use serde::{Deserialize, Serialize};

/// A SHA256 hash.
#[derive(Clone, Copy, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct H256(pub H256Hash); // big endian u256

impl H256 {
    pub fn random() -> Self {
        generate_random_hash()
    }
}

impl Hashable for H256 {
    fn hash(&self) -> H256 {
        digest(&ring::digest::SHA256, &self.0).into()
    }
}

impl std::ops::Div<f64> for H256 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        hash_divide_by(&self, rhs)
    }
}

impl std::ops::Mul<f64> for H256 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        hash_multiply_by(&self, rhs)
    }
}

impl std::fmt::Display for H256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let start = if let Some(precision) = f.precision() {
            if precision >= 64 {
                0
            } else {
                32 - precision / 2
            }
        } else {
            0
        };
        for byte_idx in start..32 {
            write!(f, "{:>02x}", &self.0[byte_idx])?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for H256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:>02x}{:>02x}..{:>02x}{:>02x}",
            &self.0[0], &self.0[1], &self.0[30], &self.0[31]
        )
    }
}

impl std::convert::AsRef<[u8]> for H256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::convert::From<&[u8; 32]> for H256 {
    fn from(input: &[u8; 32]) -> H256 {
        let mut buffer: [u8; 32] = [0; 32];
        buffer[..].copy_from_slice(input);
        H256(buffer)
    }
}

impl std::convert::From<&H256> for [u8; 32] {
    fn from(input: &H256) -> [u8; 32] {
        let mut buffer: [u8; 32] = [0; 32];
        buffer[..].copy_from_slice(&input.0);
        buffer
    }
}

impl std::convert::From<[u8; 32]> for H256 {
    fn from(input: [u8; 32]) -> H256 {
        H256(input)
    }
}

impl std::convert::From<H256> for [u8; 32] {
    fn from(input: H256) -> [u8; 32] {
        input.0
    }
}

impl std::convert::From<Vec<u8>> for H256 {
    fn from(input: Vec<u8>) -> H256 {
        let mut raw_hash: [u8; 32] = [0; 32];
        raw_hash[0..32].copy_from_slice(input.as_ref());
        H256(raw_hash)
    }
}

impl std::convert::From<ring::digest::Digest> for H256 {
    fn from(input: ring::digest::Digest) -> H256 {
        let mut raw_hash: [u8; 32] = [0; 32];
        raw_hash[0..32].copy_from_slice(input.as_ref());
        H256(raw_hash)
    }
}

impl Ord for H256 {
    fn cmp(&self, other: &H256) -> std::cmp::Ordering {
        let self_higher = u128::from_be_bytes(self.0[0..16].try_into().unwrap());
        let self_lower = u128::from_be_bytes(self.0[16..32].try_into().unwrap());
        let other_higher = u128::from_be_bytes(other.0[0..16].try_into().unwrap());
        let other_lower = u128::from_be_bytes(other.0[16..32].try_into().unwrap());
        let higher = self_higher.cmp(&other_higher);
        match higher {
            std::cmp::Ordering::Equal => self_lower.cmp(&other_lower),
            _ => higher,
        }
    }
}

impl PartialOrd for H256 {
    fn partial_cmp(&self, other: &H256) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn generate_random_hash() -> H256 {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    let mut raw_bytes = [0; 32];
    raw_bytes.copy_from_slice(&random_bytes);
    (&raw_bytes).into()
}

pub fn hash_divide_by(input: &H256, divide: f64) -> H256 {
    let mut result_bytes = [0; 32];
    for n in 1..9 {
        let value = u32::from_be_bytes(input.0[4 * (n - 1)..4 * n].try_into().unwrap());
        //println!{"{}",value};
        let value = value as f64;
        let result = value / divide;
        let result = result as u32;
        let results: [u8; 4] = result.to_be_bytes();
        //println!{"{}",result};
        result_bytes[4 * (n - 1)] = results[0];
        result_bytes[4 * (n - 1) + 1] = results[1];
        result_bytes[4 * (n - 1) + 2] = results[2];
        result_bytes[4 * (n - 1) + 3] = results[3];
    }
    (&result_bytes).into()
}

pub fn hash_multiply_by(input: &H256, multiply: f64) -> H256 {
    let mut result_bytes = [0; 32];
    for n in 1..9 {
        let value = u32::from_be_bytes(input.0[4 * (n - 1)..4 * n].try_into().unwrap());
        //println!{"{}",value};
        let value = value as f64;
        let result = value * multiply;
        let result = result as u32;
        let results: [u8; 4] = result.to_be_bytes();
        //println!{"{}",result};
        result_bytes[4 * (n - 1)] = results[0];
        result_bytes[4 * (n - 1) + 1] = results[1];
        result_bytes[4 * (n - 1) + 2] = results[2];
        result_bytes[4 * (n - 1) + 3] = results[3];
    }
    (&result_bytes).into()
}
