/*
    Appellation: error <module>
    Created At: 2025.09.08:18:06:45
    Contrib: @FL03
*/
//! the [`error`](self) module defines custom error handling routines for the crate
#[cfg(feature = "alloc")]
use alloc::string::String;

/// a type alias for a [`Result`](core::result::Result) that employs the custom [`Error`] type
pub type Result<T = ()> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[cfg(feature = "alloc")]
    #[error("Unknown state: {0}")]
    UnknownState(String),
}
