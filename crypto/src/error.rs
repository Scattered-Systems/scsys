/*
    Appellation: error <module>
    Contrib: @FL03
*/
/// a type alias for a [`Result`] type configured to use the [`CryptoError`] type
pub type CryptoResult<T = ()> = core::result::Result<T, CryptoError>;
/// a custom error type for the `scsys-crypto` crate
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[cfg(feature = "anyhow")]
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] alloc::boxed::Box<dyn std::error::Error + Send + Sync + 'static>),
    #[cfg(feature = "std")]
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[cfg(feature = "json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "alloc")]
    #[error("Unknown error: {0}")]
    Unknown(alloc::string::String),
}

#[cfg(feature = "alloc")]
impl From<alloc::string::String> for CryptoError {
    fn from(value: alloc::string::String) -> Self {
        Self::Unknown(value)
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for CryptoError {
    fn from(value: &str) -> Self {
        Self::Unknown(value.to_string())
    }
}
