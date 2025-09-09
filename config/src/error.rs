/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! the custom error types and routines for the config crate.

/// a type alias for a [`Result`] type equipped with a custom [`Error`] type
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The [`Error`] type enumerates the handled errors within the configuration crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CoreError(scsys_core::error::Error),
    #[error("Configuration error: {0}")]
    ParseError(String),
    #[cfg(feature = "config")]
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),
    #[cfg(feature = "url")]
    #[error(transparent)]
    UrlError(#[from] url::ParseError),
}

impl From<Error> for scsys_core::error::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::CoreError(e) => e,
            e => scsys_core::error::Error::box_error(e),
        }
    }
}
