/*
    Appellation: caches <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
pub enum Cache {
    Redis { name: String, uri: String },
}

impl Default for Cache {
    fn default() -> Self {
        Self::Redis {
            name: String::new(),
            uri: String::new(),
        }
    }
}
