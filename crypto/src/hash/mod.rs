/*
    Appellation: hash <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{hasher::Hash, hashes::*, utils::*};

pub(crate) mod hasher;
pub(crate) mod hashes;

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
    /// Given a collection of elements, reduce into a single hash by updating the same hasher
    pub fn iter_hasher<T: ToString>(data: &Vec<T>) -> GenericHash {
        let mut hasher = blake3::Hasher::default();
        for i in data {
            hasher.update(i.to_string().as_bytes());
        }
        hasher.finalize().as_bytes().to_owned().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{self, distributions::Alphanumeric, Rng};

    pub fn generate_random_string(length: Option<usize>) -> String {
        std::ops::Range {
            start: 0,
            end: length.unwrap_or(12),
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

    #[test]
    fn test_iter_hasher() {
        let hashes = |i: usize| {
            std::ops::Range { start: 0, end: i }
                .map(|_| generate_random_string(None))
                .collect::<Vec<String>>()
        };
        let a = iter_hasher(&hashes(10));
        let b = iter_hasher(&hashes(12));
        assert_ne!(&a, &b)
    }
}
