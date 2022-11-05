/*
    Appellation: hash <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{hashes::*, interface::Hash, utils::*};

pub(crate) mod hashes;
pub(crate) mod interface;

pub trait Hashable {
    fn hash(&self) -> H256;
}

pub(crate) mod utils {
    use crate::GenericHash;
    use std::string::ToString;

    /// hasher implements a generic hash function wrapper around blake3
    pub fn hasher<T: ToString>(data: &T) -> GenericHash {
        blake3::hash(data.to_string().as_bytes())
            .as_bytes()
            .to_owned()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{
        self,
        distributions::{Alphanumeric, Distribution},
        Rng,
    };

    pub fn generate_random_string(length: Option<usize>) -> String {
        std::ops::Range {
            start: 0,
            end: length.unwrap_or_else(|| 12),
        }
        .map(|_| rand::thread_rng().sample(Alphanumeric) as char)
        .collect::<String>()
    }

    #[test]
    fn test_hasher() {
        let a = hasher(&generate_random_string(None));
        let b = hasher(&generate_random_string(None));
        assert_ne!(&a, &b)
    }
}
