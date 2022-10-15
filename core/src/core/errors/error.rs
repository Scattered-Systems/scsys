/*
    Appellation: error <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Error {
    Async,
    Connection,
    Default,
    Generic(String),
}

impl Default for Error {
    fn default() -> Self {
        Self::Default
    }
}
