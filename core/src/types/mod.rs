/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::direction::Direction;
#[cfg(any(feature = "alloc", feature = "std"))]
pub use self::rustic::*;
#[cfg(feature = "std")]
pub use self::std_types::*;

pub mod direction;

#[cfg(any(feature = "alloc", feature = "std"))]
mod rustic {
    #[cfg(all(feature = "alloc", no_std))]
    use alloc::boxed::Box;
    #[cfg(feature = "std")]
    use std::boxed::Box;

    pub type BoxAny = Box<dyn core::any::Any>;
}

#[cfg(feature = "std")]
mod std_types {
    use std::sync::{Arc, Mutex};

    /// Type alias for async errors
    pub type AsyncError = Box<dyn std::error::Error + Send + Sync>;
    /// Type alias for async results
    pub type AsyncResult<T = ()> = core::result::Result<T, AsyncError>;
    /// Type alias for a boxed error with send, sync, and static flags enabled
    pub type BoxError = Box<dyn std::error::Error>;
    /// Type alias for the standard result used
    pub type BoxResult<T = (), E = BoxError> = core::result::Result<T, E>;
    /// Type alias wrapping a locked, thread-safe structure with a [std::sync::Mutex] in an [std::sync::Arc]
    pub type Locked<T> = Arc<Mutex<T>>;
    /// Type alias for [std::io::Result]
    pub type IOResult<T = ()> = std::io::Result<T>;
}

pub(crate) mod prelude {
    pub use super::direction::Direction;
    #[cfg(any(feature = "alloc", feature = "std"))]
    pub use super::rustic::*;
    #[cfg(feature = "std")]
    pub use super::std_types::*;
}
