/*
    Appellation: h256 <module>
    Contrib: @FL03
*/

#[cfg(feature = "blake3")]
mod impl_blake3;
mod impl_convert;
mod impl_ops;

use crate::utils::digest_to_hash;

/// The H256Hash type is a 32-byte hash.
pub type H256Array = [u8; 32];

/// A SHA256 hash.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, transparent)
)]
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct H256(pub H256Array);

impl H256 {
    /// returns a new instance of the hash without any adjustments
    pub fn new(raw: H256Array) -> Self {
        Self(raw)
    }
    /// returns a new instance of the hash from a digest
    pub fn from_digest<D>(digest: D) -> Self
    where
        D: AsRef<[u8]>,
    {
        let hash = digest_to_hash::<32>(digest);
        Self::new(hash)
    }
    /// hash some data using the [`blake3`] algorithm
    #[cfg(feature = "blake3")]
    pub fn b3(data: impl AsRef<[u8]>) -> Self {
        let hash = blake3::hash(data.as_ref());
        H256(digest_to_hash::<32>(hash.as_bytes()))
    }
    /// generate a randome hash
    #[cfg(feature = "rand")]
    pub fn random() -> Self {
        let data = rand::random::<H256Array>();
        let mut raw_bytes = [0; 32];
        raw_bytes.copy_from_slice(&data);
        (&raw_bytes).into()
    }
    /// returns the hash as a byte array
    pub const fn get(&self) -> &H256Array {
        &self.0
    }
    /// returns a mutable reference to the hash as a byte array
    pub fn get_mut(&mut self) -> &mut H256Array {
        &mut self.0
    }
    /// copies the hash into the provided buffer
    pub fn set(&mut self, value: H256Array) -> &mut Self {
        self.0 = value;
        self
    }
    /// [`replace`](core::mem::replace) and return the current value with another
    pub fn replace(&mut self, value: &H256Array) -> H256Array {
        core::mem::replace(&mut self.0, *value)
    }
    /// [`swap`](core::mem::swap) the current value with another
    pub fn swap(&mut self, other: &mut Self) {
        core::mem::swap(&mut self.0, &mut other.0)
    }
    /// returns the hash as a byte array
    pub const fn as_slice(&self) -> &[u8] {
        &self.0
    }
    /// returns a mutable reference to the hash as a byte array
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
    /// copies the hash into the provided buffer
    pub fn copy_from_slice(&mut self, value: &[u8]) -> &mut Self {
        self.0.copy_from_slice(value);
        self
    }
    #[cfg(feature = "alloc")]
    /// returns a [`Vec<u8>`](alloc::vec::Vec) representation of the hash
    pub fn to_vec(&self) -> alloc::vec::Vec<u8> {
        self.get().to_vec()
    }
}

impl core::ops::Deref for H256 {
    type Target = H256Array;

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

impl Ord for H256 {
    fn cmp(&self, other: &H256) -> core::cmp::Ordering {
        let self_higher = u128::from_be_bytes(self[0..16].try_into().unwrap());
        let self_lower = u128::from_be_bytes(self[16..32].try_into().unwrap());
        let other_higher = u128::from_be_bytes(other[0..16].try_into().unwrap());
        let other_lower = u128::from_be_bytes(other[16..32].try_into().unwrap());
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
            &self[0], &self[1], &self[30], &self[31]
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
