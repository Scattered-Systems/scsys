/*
    Appellation: errors <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize,
)]
pub enum Error<T: std::error::Error> {
    AsyncError(T),
    ConnectionError,
    Default,
    Generic(String),
}

impl<T: std::error::Error> Default for Error<T> {
    fn default() -> Self {
        Self::Default
    }
}
