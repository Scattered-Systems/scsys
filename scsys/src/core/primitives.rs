/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use constants::*;
pub use types::*;

use rand::Rng;

/// A collection of time-related data structures
#[derive(Clone, Debug, PartialEq)]
pub enum Temporal {
    Bson(bson::DateTime),
    Datetime(DateTime<Utc>),
    Timestamp(i64),
    Timezone(Utc),
}

impl Temporal {
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }
    pub fn bson_datetime() -> Self {
        Self::Bson(Self::now().into())
    }
    pub fn datetime() -> Self {
        Self::Datetime(Self::now())
    }
    pub fn timestamp() -> Self {
        Self::Timestamp(Self::now().timestamp())
    }
}

impl Default for Temporal {
    fn default() -> Self {
        Self::Timestamp(Utc::now().timestamp())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Id<T = String> {
    Obj(ObjectId),
    Other(T),
    Std(u64),
}

impl Id {
    fn random_u64() -> u64 {
        let mut rnd = rand::thread_rng();
        rnd.gen::<u64>()
    }
    pub fn generate_object_id() -> Self {
        Self::Obj(ObjectId::new())
    }
    pub fn random_std() -> Self {
        Self::Std(Self::random_u64())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::Std(Self::random_u64())
    }
}

/// Outlines a simple name tag trait for identifying different structures throughout the ecosystem
pub trait NameTag {
    fn name(&self) -> String;
    fn slug(&self) -> String {
        self.name().to_lowercase()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Appellation {
    pub label: String,
    pub name: String,
    pub slug: String,
    pub tags: Vec<String>,
}

impl Appellation {
    fn constructor(label: String, name: String, slug: String, tags: Vec<String>) -> Self {
        Self {
            label,
            name,
            slug,
            tags,
        }
    }
    pub fn new(label: String, name: String, tags: Vec<String>) -> Self {
        Self::constructor(label, name.clone(), Self::slug(name), tags)
    }
    pub fn slug(name: String) -> String {
        name.to_lowercase()
    }
}

impl Default for Appellation {
    fn default() -> Self {
        Self::new(String::new(), String::new(), Vec::<String>::new())
    }
}

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

mod constants {
    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Constants {
        pub difficulty_prefix: String,
        pub epoch: usize,
    }

    impl Constants {
        fn constructor(difficulty_prefix: String, epoch: usize) -> Self {
            Self {
                difficulty_prefix,
                epoch,
            }
        }
        pub fn new(difficulty_prefix: String, epoch: usize) -> Self {
            Self::constructor(difficulty_prefix, epoch)
        }
    }

    impl Default for Constants {
        fn default() -> Self {
            Self::new(DIFFICULTY_PREFIX.to_string(), EPOCH)
        }
    }

    ///
    pub const DIFFICULTY_PREFIX: &str = "00";
    ///
    pub const EPOCH: usize = 16;
}

mod types {
    pub use bson::{oid::ObjectId, DateTime as BsonDateTime};
    pub use chrono::{DateTime, Utc};
    pub use config::{
        builder::{AsyncState as AsyncConfigState, DefaultState as ConfigDefaultState},
        AsyncConfigBuilder, Config, ConfigBuilder, ConfigError,
    };

    /// Type alias for a common block hash
    pub type BlockHs = String;
    /// Type alias for a common block id
    pub type BlockId = u64;
    /// Type alias for a common block nonce
    pub type BlockNc = u64;
    /// Type alias for a common block timestamp
    pub type BlockTs = i64;
    /// Type alias for a common block timezone
    pub type BlockTz = Utc;
    /// Type alias for a batch of transactions for a block as an array of type (T) and size 16
    pub type BlockTr<T = String> = [T; 16];
    /// Type alias for a [Box<dyn std::error::Error>] with Send, Sync, and static flags
    pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
    /// Type alias for a vector of configuration files [Vec<config::File>]
    pub type ConfigFileVec = Vec<config::File<config::FileSourceFile, config::FileFormat>>;
    /// Type alias implementing a [ConfigBuilder] in its default state
    pub type DefaultConfigBuilder = ConfigBuilder<ConfigDefaultState>;
    /// Type alias for an [std::collections::HashMap] with a String key and variable value
    pub type Dictionary<T = String> = std::collections::HashMap<String, T>;
    // Describes a boxed dynamic error
    pub type StdError = Box<dyn std::error::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ids() {
        let actual = Id::generate_object_id();
        let expected = actual.clone();
        assert_eq!(&actual, &expected)
    }

    #[test]
    fn test_prefix() {
        let actual = Prefix::default();
        let expected = Prefix::from("");
        assert_eq!(&actual, &expected)
    }
}
