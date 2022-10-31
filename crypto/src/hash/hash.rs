/*
    Appellation: hashes <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{
    hash::{hasher, hashes::H256, Hashable},
    GenericHash,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Hash(pub GenericHash);

impl Hash {
    pub fn new<T: ToString>(data: T) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        Self(hasher.finalize())
    }
}

impl Hashable for Hash {
    fn hash(&self) -> H256 {
        self.clone().into()
    }
}

impl std::convert::From<&Hash> for Hash {
    fn from(data: &Hash) -> Self {
        Self::new(data.clone())
    }
}

impl<T: Serialize> std::convert::From<&T> for Hash {
    fn from(data: &T) -> Self {
        Self(hasher(data))
    }
}

impl std::convert::Into<Vec<u8>> for Hash {
    fn into(self) -> Vec<u8> {
        self.0.as_slice().to_owned()
    }
}

impl std::convert::Into<H256> for Hash {
    fn into(self) -> H256 {
        H256::from(self.0.as_slice().to_owned())
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
