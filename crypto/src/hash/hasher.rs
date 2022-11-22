/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#![allow(clippy::from_over_into)]
use crate::hash::{hasher, hashes::H256};
use crate::{GenericHash, Hashable};


#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Hash(pub GenericHash);

impl Hash {
    pub fn new<T>(data: T) -> Self where T: ToString {
        Self(hasher(&data))
    }
}

impl Hashable for Hash {
    fn hash(&self) -> H256 {
        self.clone().into()
    }
}

impl std::convert::From<GenericHash> for Hash {
    fn from(data: GenericHash) -> Self {
        Self(data)
    }
}

impl std::convert::From<&Hash> for Hash {
    fn from(data: &Hash) -> Self {
        Self::new(data.clone())
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
