/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[allow(dead_code)]
/// a type alias for a [`Result`] type equipped with a custom [`Error`] type
pub(crate) type Result<T = ()> = core::result::Result<T, ConfigError>;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration error: {0}")]
    ParseError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[cfg(feature = "config")]
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync + 'static>),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[cfg(feature = "json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    UrlError(#[from] url::ParseError),
}
