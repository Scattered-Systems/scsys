/*
    Appellation: epoch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Timestamp;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Epoch<T = u128> {
    size: u128,
    timestamp: Timestamp<T>,
}

impl<T> Epoch<T> {
    pub fn new(size: u128, timestamp: Timestamp<T>) -> Self {
        Self { size, timestamp }
    }
    /// The size of the epoch; epochs generally consider the number of steps, or size,
    /// (e.g. the number of seconds) that will need to be taken before the epoch is considered
    /// to be complete and a new one begins.
    pub fn size(&self) -> u128 {
        self.size
    }

    pub fn timestamp(&self) -> &T {
        &self.timestamp
    }

    pub fn set_size(&mut self, size: u128) {
        self.size = size;
    }
}
