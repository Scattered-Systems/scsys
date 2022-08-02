/*
    Appellation: mod <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use constants::*;
pub use types::*;
pub use variants::*;

pub use bson::{oid::ObjectId, DateTime as BsonDateTime};
pub use chrono::{DateTime, Utc};
pub use config::{
    builder::{AsyncState as AsyncConfigState, DefaultState as ConfigDefaultState},
    AsyncConfigBuilder, Config, ConfigBuilder, ConfigError, File as ConfigFile,
};
pub use serde::{Deserialize, Serialize};

mod constants;
mod types;
mod variants;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Prefix(String);

impl Prefix {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn from<T: Clone + std::string::ToString>(data: T) -> Self {
        Self::new(data.to_string())
    }
}

impl Default for Prefix {
    fn default() -> Self {
        Self::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::Prefix;

    #[test]
    fn test_prefix() {
        let actual = Prefix::default();
        let expected = Prefix::from("");
        assert_eq!(&actual, &expected)
    }
}
