/*
    Appellation: time <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Time
//!
//! The `time` module provides a set of utilities for working with time and timestamps.
#[doc(inline)]
#[cfg(feature = "std")]
pub use self::utils::*;
#[doc(inline)]
pub use self::{timestamp::Timestamp, traits::*, types::*};
/// this module implements the [`Timestamp`] type
pub mod timestamp;

mod impls {
    pub mod impl_timestamp;
    pub mod impl_timestamp_repr;
}

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
    #[doc(inline)]
    pub use self::prelude::*;

    #[cfg(feature = "std")]
    mod base;

    mod prelude {
        #[doc(inline)]
        #[cfg(feature = "std")]
        pub use super::base::*;
    }
}
