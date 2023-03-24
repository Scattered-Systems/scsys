/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[strum(serialize_all = "snake_case")]
pub enum Error {
    AsyncError(String),
    #[default]
    Custom(String),
    ConnectionError,
    DecodeError,
    EncodeError,
    Error(String),
    ExtractionError,
    IOError(String),
    InitalizationError,
    ParseError,
    TerminationError,
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Custom(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error.into())
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Error::Error(error.into())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Error::AsyncError(error.into())
    }
}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn test_error_default() {
        let actual = Error::default();
        assert_eq!(actual, Error::Custom(Default::default()))
    }

    #[test]
    fn test_error_std() {
        let err: Box<dyn std::error::Error> = "test".into();

        assert_eq!(Error::from(err), Error::Error("test".into()))
    }
}
