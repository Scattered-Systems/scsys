/*
    Appellation: hasher <module>
    Contrib: @FL03
*/

/// A [`Hasher`] defines a common interface for defining specific hashing algorithm.
pub trait Hasher {
    /// the output type of the hasher
    type Output;
    /// reset the hasher to its initial state
    fn clean(&mut self) -> &mut Self;
    /// finalize the hash and return the result
    fn finish(&self) -> crate::CryptoResult<Self::Output>;
    /// update the hasher with new data
    fn include(&mut self, data: impl AsRef<[u8]>) -> &mut Self;
}

#[cfg(feature = "blake3")]
mod impl_blake3 {
    use crate::hash::H256;

    impl super::Hasher for blake3::Hasher {
        type Output = H256;

        fn clean(&mut self) -> &mut Self {
            self.reset()
        }

        fn finish(&self) -> crate::CryptoResult<Self::Output> {
            // finalize the `blake3` hasher
            let hash = self.finalize();
            // convert the hash into the defined type before returning it
            Ok(hash.into())
        }

        fn include(&mut self, data: impl AsRef<[u8]>) -> &mut Self {
            self.update(data.as_ref())
        }
    }
}
