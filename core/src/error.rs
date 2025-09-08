/*
    Appellation: error <module>
    Created At: 2025.09.08:18:06:45
    Contrib: @FL03
*/
//! ths module implements various error-handling primitives and utilities
//!
#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String};

/// a type alias for a [`Result`] type pre-configured with the [`Error`] type
pub type Result<T = ()> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync + 'static>),
    #[error(transparent)]
    FmtError(#[from] core::fmt::Error),
    #[cfg(feature = "std")]
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[cfg(feature = "json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "alloc")]
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::Error;
    use alloc::{boxed::Box, string::String};

    impl Error {
        pub fn box_error<E>(error: E) -> Self
        where
            E: core::error::Error + Send + Sync + 'static,
        {
            Self::BoxError(Box::new(error))
        }
    }

    impl From<String> for Error {
        fn from(value: String) -> Self {
            Self::Unknown(value)
        }
    }

    impl From<&str> for Error {
        fn from(value: &str) -> Self {
            Self::Unknown(String::from(value))
        }
    }
}
