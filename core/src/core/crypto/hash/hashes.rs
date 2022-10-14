/*
    Appellation: hashes <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::{H160, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Hashes {
    H256(H256),
    H160(H160),
}

impl Default for Hashes {
    fn default() -> Self {
        Self::H256(H256::default())
    }
}
