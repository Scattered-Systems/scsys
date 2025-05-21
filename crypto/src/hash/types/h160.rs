/*
    Appellation: h160 <module>
    Contrib: @FL03
*/
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
pub struct H160(pub [u8; 20]);

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
    /// returns the hash as a byte array
    pub const fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }
    /// returns a mutable reference to the hash as a byte array
    pub fn as_mut_bytes(&mut self) -> &mut [u8; 20] {
        &mut self.0
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

impl From<[u8; 20]> for H160 {
    #[inline]
    fn from(value: [u8; 20]) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a [u8; 20]> for H160 {
    #[inline]
    fn from(value: &'a [u8; 20]) -> Self {
        Self(*value)
    }
}

impl From<H160> for [u8; 20] {
    #[inline]
    fn from(value: H160) -> Self {
        value.0
    }
}
impl<'a> From<&'a H160> for &'a [u8; 20] {
    #[inline]
    fn from(value: &'a H160) -> Self {
        &value.0
    }
}
