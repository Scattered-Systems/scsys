/*
    Appellation: error <module>
    Created At: 2025.09.08:17:20:05
    Contrib: @FL03
*/
//! the [`error`](self) module provides a custom error type for the `time` crate
//! as well as a type alias for results using this error type.

/// A type alias for a [`Result`](core::result::Result) with the error type set to [`Error`].
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An error occurred with the timestamp: {0}")]
    TimestampError(&'static str),
}
