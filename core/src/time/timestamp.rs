/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// Timestamp implements a host of useful utilities for stamping data
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[repr(transparent)]
pub struct Timestamp(u128);

impl Timestamp {
    /// Create a new timestamp
    pub fn now() -> Self {
        Self(crate::time::systime())
    }

    pub const fn timestamp(&self) -> u128 {
        self.0
    }
}

impl AsRef<u128> for Timestamp {
    fn as_ref(&self) -> &u128 {
        &self.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl Deref for Timestamp {
    type Target = u128;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u128> for Timestamp {
    fn from(timestamp: u128) -> Self {
        Self(timestamp)
    }
}

impl From<Timestamp> for u128 {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let a = Timestamp::now();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let b = Timestamp::now();
        assert_ne!(a, b);
        assert!(a < b);
    }
}
