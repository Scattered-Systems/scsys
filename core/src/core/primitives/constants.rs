/*
    Appellation: constants <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Constants {
    pub difficulty_prefix: String,
    pub epoch: usize,
}

impl Constants {
    fn constructor(difficulty_prefix: String, epoch: usize) -> Self {
        Self {
            difficulty_prefix,
            epoch,
        }
    }
    pub fn new(difficulty_prefix: String, epoch: usize) -> Self {
        Self::constructor(difficulty_prefix, epoch)
    }
}

impl Default for Constants {
    fn default() -> Self {
        Self::new(DIFFICULTY_PREFIX.to_string(), EPOCH)
    }
}

///
pub const DIFFICULTY_PREFIX: &str = "00";
///
pub const EPOCH: usize = 16;
