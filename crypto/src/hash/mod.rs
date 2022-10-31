/*
    Appellation: hash <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{hash::Hash, hashes::*, utils::*};

pub(crate) mod hash;
pub(crate) mod hashes;

pub trait Hashable {
    fn hash(&self) -> H256;
}

pub(crate) mod utils {
    use crate::GenericHash;
    use sha2::{Digest, Sha256};
    use std::string::ToString;

    pub fn hasher<T: ToString>(data: &T) -> GenericHash {
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        hasher.finalize()
    }
}
