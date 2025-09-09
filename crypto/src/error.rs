/*
    Appellation: error <module>
    Contrib: @FL03
*/
//! this module provides error handling utilities for the `scsys-crypto` crate

/// a type alias for a [`Result`](core::result::Result) with the error type fixed to [`Error`]
pub type Result<T = ()> = core::result::Result<T, Error>;

/// a custom error type for the `scsys-crypto` crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CoreError(scsys_core::error::Error),
    #[error("An unhashable object was provided")]
    Unhashable,
}

impl From<Error> for scsys_core::error::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::CoreError(e) => e,
            e => scsys_core::error::Error::box_error(e),
        }
    }
}
