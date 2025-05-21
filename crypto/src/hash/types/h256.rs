/*
    Appellation: h256 <module>
    Contrib: @FL03
*/
use super::H160;
use crate::types::GenericHash;
use crate::utils::digest_to_hash;

/// The H256Hash type is a 32-byte hash.
pub type H256Hash = [u8; 32];

/// A SHA256 hash.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, transparent)
)]
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct H256(pub [u8; 32]);

impl H256 {
    #[cfg(feature = "blake3")]
    pub fn b3(data: impl AsRef<[u8]>) -> Self {
        let hash = blake3::hash(data.as_ref());
        H256(digest_to_hash::<32>(hash.as_bytes()))
    }
    #[cfg(feature = "rand")]
    pub fn generate() -> Self {
        let data = rand::random::<[u8; 32]>();
        let mut raw_bytes = [0; 32];
        raw_bytes.copy_from_slice(&data);
        (&raw_bytes).into()
    }
    /// returns the hash as a byte array
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
    /// returns a mutable reference to the hash as a byte array
    pub fn as_mut_bytes(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }
}

impl core::ops::Deref for H256 {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for H256 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsMut<[u8]> for H256 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl AsRef<[u8]> for H256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8; 32]> for H256 {
    fn as_mut(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }
}

impl AsRef<[u8; 32]> for H256 {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

#[cfg(feature = "blake3")]
mod impl_blake3 {
    use super::H256;
    use crate::Concat;

    impl Concat for H256 {
        type Output = H256;

        fn concat(&self, other: H256) -> Self {
            let mut res: Vec<u8> = (*self).into();
            let mut rnode: Vec<u8> = (*other).into();
            res.append(&mut rnode);

            blake3::hash(&res).into()
        }
    }
}

impl Ord for H256 {
    fn cmp(&self, other: &H256) -> core::cmp::Ordering {
        let self_higher = u128::from_be_bytes(self.0[0..16].try_into().unwrap());
        let self_lower = u128::from_be_bytes(self.0[16..32].try_into().unwrap());
        let other_higher = u128::from_be_bytes(other.0[0..16].try_into().unwrap());
        let other_lower = u128::from_be_bytes(other.0[16..32].try_into().unwrap());
        let higher = self_higher.cmp(&other_higher);
        match higher {
            core::cmp::Ordering::Equal => self_lower.cmp(&other_lower),
            _ => higher,
        }
    }
}

impl PartialOrd for H256 {
    fn partial_cmp(&self, other: &H256) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::fmt::Debug for H256 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{:>02x}{:>02x}..{:>02x}{:>02x}",
            &self.0[0], &self.0[1], &self.0[30], &self.0[31]
        )
    }
}

impl core::fmt::Display for H256 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
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

impl FromIterator<u8> for H256 {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let digest = iter.into_iter().collect::<Vec<u8>>();
        digest_to_hash::<32>(&digest).into()
    }
}

impl IntoIterator for H256 {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, 32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> From<&T> for H256
where
    T: AsRef<[u8]>,
{
    fn from(data: &T) -> H256 {
        let hash = blake3::hash(data.as_ref());
        let mut buffer: [u8; 32] = [0; 32];
        buffer[..].copy_from_slice(hash.as_bytes());
        H256(buffer)
    }
}

impl From<[u8; 32]> for H256 {
    fn from(input: [u8; 32]) -> H256 {
        H256(input)
    }
}

impl From<H256> for [u8; 32] {
    fn from(input: H256) -> [u8; 32] {
        input.0
    }
}

impl From<Vec<u8>> for H256 {
    fn from(input: Vec<u8>) -> H256 {
        digest_to_hash::<32>(&input).into()
    }
}

impl From<H256> for Vec<u8> {
    fn from(input: H256) -> Vec<u8> {
        input.0.to_vec()
    }
}
#[cfg(feature = "blake3")]
impl From<blake3::Hash> for H256 {
    fn from(input: blake3::Hash) -> H256 {
        let mut raw_hash: [u8; 32] = [0; 32];
        raw_hash[0..32].copy_from_slice(input.as_bytes());
        H256(raw_hash)
    }
}

#[cfg(feature = "blake3")]
impl From<H256> for blake3::Hash {
    fn from(input: H256) -> blake3::Hash {
        blake3::Hash::from(input.0)
    }
}

impl From<GenericHash> for H256 {
    fn from(data: GenericHash) -> H256 {
        data.as_slice().to_owned().into()
    }
}

impl From<H256> for GenericHash {
    fn from(input: H256) -> GenericHash {
        GenericHash::from(input.0)
    }
}

impl From<H160> for H256 {
    fn from(input: H160) -> H256 {
        let mut buffer: super::H256Hash = [0; 32];
        buffer[..].copy_from_slice(&input.0[0..20]);
        buffer.into()
    }
}

impl From<H256> for H160 {
    fn from(input: H256) -> H160 {
        let mut buffer: super::H160Hash = [0; 20];
        buffer[..].copy_from_slice(&input.0[0..20]);
        buffer.into()
    }
}

impl core::ops::Add<Self> for H256 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let results: [u8; 4] = {
                let val = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let rhs = u32::from_be_bytes(rhs.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((val as u64) + (rhs as u64)) as u32;
                tmp.to_be_bytes()
            };
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        Self(result_bytes)
    }
}

impl core::ops::AddAssign<Self> for H256 {
    fn add_assign(&mut self, rhs: Self) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let val = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let rhs = u32::from_be_bytes(rhs.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((val as u64) + (rhs as u64)) as u32;
                tmp.to_be_bytes()
            };
            self[4 * (n - 1)] = results[0];
            self[4 * (n - 1) + 1] = results[1];
            self[4 * (n - 1) + 2] = results[2];
            self[4 * (n - 1) + 3] = results[3];
        }
    }
}

impl core::ops::Add<f64> for H256 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) + rhs) as u32;
                tmp.to_be_bytes()
            };
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        Self(result_bytes)
    }
}

impl core::ops::AddAssign<f64> for H256 {
    fn add_assign(&mut self, rhs: f64) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) + rhs) as u32;
                tmp.to_be_bytes()
            };
            self[4 * (n - 1)] = results[0];
            self[4 * (n - 1) + 1] = results[1];
            self[4 * (n - 1) + 2] = results[2];
            self[4 * (n - 1) + 3] = results[3];
        }
    }
}

impl core::ops::Div<f64> for H256 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) / rhs) as u32;
                tmp.to_be_bytes()
            };
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        Self(result_bytes)
    }
}

impl core::ops::DivAssign<f64> for H256 {
    fn div_assign(&mut self, rhs: f64) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) / rhs) as u32;
                tmp.to_be_bytes()
            };
            self[4 * (n - 1)] = results[0];
            self[4 * (n - 1) + 1] = results[1];
            self[4 * (n - 1) + 2] = results[2];
            self[4 * (n - 1) + 3] = results[3];
        }
    }
}

impl core::ops::Mul<f64> for H256 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) * rhs) as u32;
                tmp.to_be_bytes()
            };
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        Self(result_bytes)
    }
}

impl core::ops::MulAssign<f64> for H256 {
    fn mul_assign(&mut self, rhs: f64) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) * rhs) as u32;
                tmp.to_be_bytes()
            };
            self[4 * (n - 1)] = results[0];
            self[4 * (n - 1) + 1] = results[1];
            self[4 * (n - 1) + 2] = results[2];
            self[4 * (n - 1) + 3] = results[3];
        }
    }
}

impl core::ops::Index<usize> for H256 {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl core::ops::IndexMut<usize> for H256 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl core::ops::Index<core::ops::Range<usize>> for H256 {
    type Output = [u8];

    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl core::ops::IndexMut<core::ops::Range<usize>> for H256 {
    fn index_mut(&mut self, index: core::ops::Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}
