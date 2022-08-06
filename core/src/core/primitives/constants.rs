/*
    Appellation: constants <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct Constants {
    pub difficulty_prefix: String,
    pub epoch: usize,
}

impl Default for Constants {
    fn default() -> Self {
        Self {
            difficulty_prefix: DIFFICULTY_PREFIX.to_string(),
            epoch: EPOCH,
        }
    }
}

///
pub const DIFFICULTY_PREFIX: &str = "00";
///
pub const EPOCH: usize = 16;
