/*
    Appellation: h160 <module>
    Contrib: @FL03
*/
#[cfg(feature = "blake3")]
mod impl_blake3;
mod impl_convert;
mod impl_ops;

/// The H160Hash type is a 20-byte hash.
pub type H160Hash = [u8; 20];

/// A SHA160 hash type.
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, transparent)
)]
#[repr(transparent)]
pub struct H160(pub H160Hash);

impl H160 {
    pub fn from_digest(digest: impl AsRef<[u8]>) -> Self {
        let hash = crate::digest_to_hash::<20>(digest);
        Self(hash)
    }
    #[cfg(feature = "blake3")]
    pub fn b3(data: impl AsRef<[u8]>) -> Self {
        let hash = blake3::hash(data.as_ref());
        Self::from_digest(hash.as_bytes())
    }
    #[cfg(feature = "rand")]
    pub fn random() -> Self {
        let data = rand::random::<H160Hash>();
        let mut raw_bytes = [0; 20];
        raw_bytes.copy_from_slice(&data);
        (&raw_bytes).into()
    }
    /// returns the hash as a slice
    pub const fn as_slice(&self) -> &[u8] {
        &self.0
    }
    /// returns a mutable reference to the hash as a slice
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
    /// copies the hash into the provided buffer
    pub fn copy_from_slice(&mut self, value: &[u8]) -> &mut Self {
        self.0.copy_from_slice(value);
        self
    }
    /// returns the hash as a byte array
    pub const fn get(&self) -> &H160Hash {
        &self.0
    }
    /// returns a mutable reference to the hash as a byte array
    pub const fn get_mut(&mut self) -> &mut H160Hash {
        &mut self.0
    }
    /// updates the hash with the provided value
    pub fn set(&mut self, value: H160Hash) -> &mut Self {
        *self.get_mut() = value;
        self
    }
    /// [`replace`](core::mem::replace) and return the current value with another
    pub fn replace(&mut self, value: &H160Hash) -> H160Hash {
        core::mem::replace(&mut self.0, *value)
    }
    /// [`swap`](core::mem::swap) the current value with another
    pub fn swap(&mut self, other: &mut Self) {
        core::mem::swap(&mut self.0, &mut other.0)
    }
    #[cfg(feature = "alloc")]
    /// returns a [`Vec<u8>`](alloc::vec::Vec) representation of the hash
    pub fn to_vec(&self) -> alloc::vec::Vec<u8> {
        self.get().to_vec()
    }
}

impl core::fmt::Debug for H160 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{:>02x}{:>02x}..{:>02x}{:>02x}",
            &self[0], &self[1], &self[18], &self[19]
        )
    }
}

impl core::fmt::Display for H160 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let start = if let Some(precision) = f.precision() {
            if precision >= 64 {
                0
            } else {
                20 - precision / 2
            }
        } else {
            0
        };
        for byte_idx in start..20 {
            write!(f, "{:>02x}", &self.0[byte_idx])?;
        }
        Ok(())
    }
}

impl AsRef<[u8]> for H160 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for H160 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl core::borrow::Borrow<[u8]> for H160 {
    #[inline]
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl core::borrow::BorrowMut<[u8]> for H160 {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl core::ops::Deref for H160 {
    type Target = [u8; 20];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for H160 {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
