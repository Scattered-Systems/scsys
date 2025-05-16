/*
    Appellation: core_error <module>
    Contrib: @FL03
*/
#[cfg(feature = "alloc")]
use alloc::string::String;

/// a type alias for a [`Result`] type pre-configured with the [`Error`] type
pub type Result<T = ()> = core::result::Result<T, CoreError>;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[cfg(feature = "anyhow")]
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
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
impl From<alloc::string::String> for CoreError {
    fn from(value: alloc::string::String) -> Self {
        Self::Unknown(value)
    }
}
#[cfg(feature = "alloc")]
impl From<&str> for CoreError {
    fn from(value: &str) -> Self {
        Self::Unknown(value.to_string())
    }
}