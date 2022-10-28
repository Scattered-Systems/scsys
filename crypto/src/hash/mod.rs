/*
    Appellation: hash <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{hash::Hash, hashes::*, utils::*};

pub(crate) mod hash;
pub(crate) mod hashes;

pub(crate) mod utils {
    use crate::HashGeneric;
    use serde::Serialize;
    use sha2::{Digest, Sha256};

    pub fn hasher<T: Serialize>(data: &T) -> HashGeneric {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(data).unwrap().as_bytes());
        hasher.finalize()
    }
}
