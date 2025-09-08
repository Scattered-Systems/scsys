/*
    Appellation: epoch <module>
    Created At: 2025.09.08:17:31:13
    Contrib: @FL03
*/
use crate::timestamp::Timestamp;
use crate::traits::RawTimestamp;

/// An [`Epoch`] represents a span of time defined by a starting [`Timestamp`] and a size in
/// units of time (e.g., seconds, minutes, hours). The size indicates the duration of the epoch
/// and is used to determine when the epoch ends.
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
    /// returns a new instance of the [`Epoch`] using the given size and timestamp.
    pub const fn new(size: u128, timestamp: Ts) -> Self {
        Self {
            size,
            timestamp: Timestamp(timestamp),
        }
    }
    /// The size of the epoch; epochs generally consider the number of steps, or size,
    /// (e.g. the number of seconds) that will need to be taken before the epoch is considered
    /// to be complete and a new one begins.
    pub const fn size(&self) -> u128 {
        self.size
    }
    /// returns an immutable reference to the timestamp
    pub const fn timestamp(&self) -> &Timestamp<Ts> {
        &self.timestamp
    }
    /// update the size of the epoch
    pub fn set_size(&mut self, size: u128) {
        self.size = size;
    }
    /// consumes the current instance to create another with the given size and current
    /// timestamp.
    pub fn with_size(self, size: u128) -> Self {
        Self { size, ..self }
    }
    /// consumes the current instance to create another with the given timestamp and current
    /// size.
    pub fn with_timestamp<U: RawTimestamp>(self, timestamp: U) -> Epoch<U> {
        Epoch {
            timestamp: Timestamp(timestamp),
            size: self.size,
        }
    }
}
