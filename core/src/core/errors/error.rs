/*
    Appellation: error <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Default, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
pub enum Error {
    AsyncError,
    ConnectionError,
    #[default]
    Generic,
}

