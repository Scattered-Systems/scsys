/*
    Appellation: error <module>
    Created At: 2025.09.08:17:20:05
    Contrib: @FL03
*/
//! ths module implements various error-handling primitives and utilities
//!

/// A type alias for a [`Result`](core::result::Result) with the error type set to [`Error`].
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    

}