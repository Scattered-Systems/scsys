/*
    Appellation: epoch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::time::{RawTimestamp, Timestamp};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Epoch<Ts = u128>
where
    Ts: RawTimestamp,
{
    size: u128,
    timestamp: Timestamp<Ts>,
}

impl<Ts> Epoch<Ts>
where
    Ts: RawTimestamp,
{
    pub fn new(size: u128, timestamp: Timestamp<Ts>) -> Self {
        Self { size, timestamp }
    }
    /// The size of the epoch; epochs generally consider the number of steps, or size,
    /// (e.g. the number of seconds) that will need to be taken before the epoch is considered
    /// to be complete and a new one begins.
    pub fn size(&self) -> u128 {
        self.size
    }

    pub fn timestamp(&self) -> &Ts {
        &self.timestamp
    }

    pub fn set_size(&mut self, size: u128) {
        self.size = size;
    }
}
