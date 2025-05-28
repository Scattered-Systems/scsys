/*
    Appellation: time <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Time
//!
//! The `time` module provides a set of utilities for working with time and timestamps.
#[doc(inline)]
#[cfg(feature = "std")]
pub use self::utils::{std_time, systime};
#[doc(inline)]
pub use self::{timestamp::Timestamp, types::prelude::*};
/// this module implements the [`Timestamp`] type
pub mod timestamp;
/// this module contains various implementations used to support `time` related features
pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod datetime;
    pub mod epoch;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::datetime::*;
        #[doc(inline)]
        pub use super::epoch::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::timestamp::*;
    #[doc(inline)]
    pub use super::types::prelude::*;
    #[cfg(feature = "std")]
    pub use super::utils::{std_time, systime};
    #[doc(inline)]
    pub use super::{Now, RawTimestamp};
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
        systime().as_millis()
    }
}

/// The [`Now`] trait provides a common creation routines for all datetime implementations.
pub trait Now {
    type Output;

    fn now() -> Self::Output;
}

/// a private trait used to mark types capable of being uses as a basetype for a [`Timestamp`].
pub trait RawTimestamp {
    private!();
}

/*
 ************* Implementations *************
*/
macro_rules! impl_raw_timestamp {
    ($($t:ty),* $(,)?) => {
        $(
            impl_raw_timestamp!(@impl $t);
        )*
    };
    (@impl $T:ty) => {
        impl RawTimestamp for $T {
            seal!();
        }
    };
}

impl_raw_timestamp! {
    u64,
    u128,
    i64,
}

impl<'a> RawTimestamp for &'a str {
    seal!();
}

#[cfg(feature = "alloc")]
impl RawTimestamp for alloc::string::String {
    seal!();
}

#[cfg(feature = "std")]
impl Now for u64 {
    type Output = Timestamp<Self>;

    fn now() -> Self::Output {
        Timestamp::new(utils::systime().as_secs())
    }
}

#[cfg(feature = "std")]
impl Now for u128 {
    type Output = Timestamp<Self>;

    fn now() -> Self::Output {
        Timestamp::new(utils::systime().as_millis())
    }
}

#[cfg(feature = "chrono")]
impl Now for i64 {
    type Output = Timestamp<Self>;

    fn now() -> Self::Output {
        Timestamp::new(chrono::Local::now().timestamp())
    }
}

#[cfg(feature = "chrono")]
impl Now for alloc::string::String {
    type Output = Timestamp<Self>;

    fn now() -> Self::Output {
        Timestamp::new(chrono::Local::now().to_rfc3339())
    }
}
