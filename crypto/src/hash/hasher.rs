/*
    Appellation: hasher <module>
    Contrib: @FL03
*/

/// a generic interface for all compatible hashers
pub trait Hasher {
    type Hash;
    /// initialize the hasher
    fn hash(data: impl AsRef<[u8]>) -> Self::Hash;
    /// finalize the hash and return the result
    fn finalize(&self) -> crate::CryptoResult<Self::Hash>;
    /// reset the hasher to its initial state
    fn reset(&mut self) -> &mut Self;
    /// update the hasher with new data
    fn update(&mut self, data: impl AsRef<[u8]>) -> &mut Self;
}
