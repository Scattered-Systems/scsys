/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::direction::Direction;
#[cfg(feature = "std")]
#[doc(inline)]
pub use self::std_types::*;

pub mod direction;

pub(crate) mod prelude {
    pub use super::direction::Direction;
    #[cfg(feature = "std")]
    pub use super::std_types::*;
}

#[cfg(feature = "std")]
mod std_types {
    /// Type alias for async errors
    pub type AsyncError = Box<dyn std::error::Error + Send + Sync>;
    /// Type alias for async results
    pub type AsyncResult<T = ()> = core::result::Result<T, AsyncError>;
    /// Type alias for a boxed error with send, sync, and static flags enabled
    pub type BoxError = Box<dyn core::error::Error>;
    /// Type alias for the standard result used
    pub type BoxResult<T = (), E = BoxError> = core::result::Result<T, E>;
    /// Type alias wrapping a locked, thread-safe structure with a [Mutex] in an [Arc]
    pub type Arcm<T> = std::sync::Arc<std::sync::Mutex<T>>;
    /// Type alias for [std::io::Result]
    pub type IOResult<T = ()> = std::io::Result<T>;
}

#[allow(unused_imports)]
pub(crate) mod rs {
    pub(crate) use core::*;

    #[cfg(all(feature = "alloc", no_std))]
    pub(crate) use self::no_std::*;
    #[cfg(feature = "std")]
    pub(crate) use self::std_ty::*;

    #[cfg(feature = "alloc")]
    pub(crate) mod no_std {
        pub(crate) use alloc::boxed::{self, Box};
        pub(crate) use alloc::collections::{self, BTreeMap, BTreeSet};
        pub(crate) use alloc::string::{self, String, ToString};
        pub(crate) use alloc::sync::{self, Arc};
        pub(crate) use alloc::vec::{self, Vec};
    }

    #[cfg(feature = "std")]
    pub(crate) mod std_ty {
        pub(crate) use std::boxed::{self, Box};
        pub(crate) use std::collections::{self, BTreeMap, BTreeSet};
        pub(crate) use std::string::{self, String, ToString};
        pub(crate) use std::sync::{self, Arc};
        pub(crate) use std::vec::{self, Vec};
    }
}
