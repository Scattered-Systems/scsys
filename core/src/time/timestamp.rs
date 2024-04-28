/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::borrow::Borrow;
use core::ops::Deref;

/// Timestamp implements a host of useful utilities for stamping data
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct Timestamp(u128);

impl Timestamp {
    pub fn new(timestamp: u128) -> Self {
        Self(timestamp)
    }

    /// Create a new timestamp
    pub fn now() -> Self {
        Self(crate::time::systime())
    }

    pub const fn get(&self) -> u128 {
        self.0
    }
}

impl AsRef<u128> for Timestamp {
    fn as_ref(&self) -> &u128 {
        &self.0
    }
}

impl Borrow<u128> for Timestamp {
    fn borrow(&self) -> &u128 {
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

macro_rules! fmt_timestamp {
    ($($t:ident($($rest:tt)*)),*) => {
        $(
            impl core::fmt::$t for Timestamp {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    write!(f, $($rest)*, self.0)
                }
            }
        )*
    };
}

fmt_timestamp! {
    Binary("{:b}"),
    Display("{}"),
    LowerExp("{:e}"),
    LowerHex("{:x}"),
    Octal("{:o}"),
    UpperExp("{:E}"),
    UpperHex("{:X}")
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
