/*
    Appellation: time <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! The [`time`](self) module works to provide a set of common interface for working with
//! temporal structures.
#[doc(inline)]
#[cfg(feature = "std")]
pub use self::utils::*;
#[doc(inline)]
pub use self::{timestamp::Timestamp, traits::*, types::*};
/// this module implements the [`Timestamp`] type
pub mod timestamp;

mod types {
    //! this module contains various implementations used to support `time` related features
    #[doc(inline)]
    pub use self::prelude::*;

    mod datetime;
    mod epoch;

    mod prelude {
        #[doc(inline)]
        pub use super::datetime::*;
        #[doc(inline)]
        pub use super::epoch::*;
    }
}

mod traits {
    //! this moodule implements the core traits supporting the `time` module
    #[doc(inline)]
    pub use self::prelude::*;

    mod now;
    mod timestamp;

    mod prelude {
        #[doc(inline)]
        pub use super::now::*;
        #[doc(inline)]
        pub use super::timestamp::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::timestamp::*;
    #[doc(inline)]
    pub use super::traits::*;
    #[doc(inline)]
    pub use super::types::*;
    #[cfg(feature = "std")]
    pub use super::utils::*;
}

mod utils {
    #[cfg(feature = "std")]
    /// [systime] is a utilitarian function that returns the current system time in milliseconds.
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
