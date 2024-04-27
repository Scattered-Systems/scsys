/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # types
//!
//!

pub use self::utils::*;

#[cfg(feature = "std")]
pub use std_types::*;

pub type AnyAsync = Box<dyn core::any::Any + Send + Sync + 'static>;

/// A type alias for [core::result::Result] that employs the [crate::errors::Error] type
pub type Result<T = ()> = core::result::Result<T, crate::errors::Error>;

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

pub(crate) mod utils {
    use core::any::{Any, TypeId};
    use core::str::FromStr;

    /// Checks to see if the input is a string
    pub fn is_string<T: ?Sized + Any>(_s: &T) -> bool {
        TypeId::of::<String>() == TypeId::of::<T>()
    }
    /// Simple function wrapper evaluating the claim that the given information is of type f64
    pub fn is_float<T>(_val: &T) -> bool
    where
        T: Any + ?Sized,
    {
        TypeId::of::<f64>() == TypeId::of::<T>()
    }

    /// Simple function wrapper evaluating the claim that the given information is of type f64
    pub fn is_str_float<T: ToString>(data: &T) -> bool {
        f64::from_str(&data.to_string()).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_string() {
        let s = "hello";
        assert!(!is_string(&s));
        assert!(is_string(&s.to_string()))
    }
}
