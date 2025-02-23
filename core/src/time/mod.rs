/*
    Appellation: time <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Time
//!
//! The `time` module provides a set of utilities for working with time and timestamps.
#[allow(unused_imports)]
#[doc(inline)]
pub use self::{epoch::Epoch, timestamp::Timestamp, utils::*};

#[doc(hidden)]
pub mod datetime;
pub mod epoch;
pub mod timestamp;

pub(crate) mod prelude {
    pub use super::Now;
    pub use super::epoch::Epoch;
    pub use super::timestamp::Timestamp;
    #[allow(unused_imports)]
    pub use super::utils::*;
}

///
pub trait Now {
    type Output;

    fn now() -> Self::Output;
}

pub(crate) mod utils {
    /// [systime] is a utilitarian function that returns the current system time in milliseconds.
    #[cfg(feature = "std")]
    #[inline]
    pub fn systime() -> core::time::Duration {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
    }
    /// [systime] is a utilitarian function that returns the current system time in milliseconds.
    #[cfg(feature = "std")]
    #[inline]
    pub fn std_time() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}
