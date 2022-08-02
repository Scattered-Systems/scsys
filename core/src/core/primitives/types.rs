/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
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
