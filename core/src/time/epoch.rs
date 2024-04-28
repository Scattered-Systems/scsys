/*
    Appellation: epoch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize,))]
pub struct Epoch {
    size: u128,
    timestamp: i64,
}

impl Epoch {
    pub fn new(size: u128, timestamp: i64) -> Self {
        Self { size, timestamp }
    }
    pub fn size(&self) -> u128 {
        self.size
    }
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}
