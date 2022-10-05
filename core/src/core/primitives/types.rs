/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::blocks::*;

pub use config::{AsyncConfigBuilder, ConfigBuilder, ConfigError};

/// Type alias for a boxed standard error
pub type BaseError = Box<dyn std::error::Error>;
/// Type alias of a result implementing the [BaseError]
pub type BaseResult<T = ()> = Result<T, super::BaseError>;
/// Type alias for a boxed error with send, sync, and static flags enabled
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
/// Type alias for the standard result used
pub type BoxResult<T = ()> = Result<T, BoxError>;
/// Type alias for [bson::DateTime]
pub type BsonDateTime = bson::DateTime;
/// Type alias for [bson::oid::ObjectId]
pub type BsonOid = bson::oid::ObjectId;
/// Type alias for [config::Environment]
pub type ConfigEnvironment = config::Environment;
/// Type alias for [config::File]
pub type ConfigFile<Src, Fmt> = config::File<Src, Fmt>;
/// Type alias for a collection of [crate::ConfigFile]
pub type ConfigFileVec = Vec<ConfigFile<config::FileSourceFile, config::FileFormat>>;
/// Type alias for a configuration result
pub type ConfigResult<T> = Result<T, ConfigError>;
/// Type alias for an [std::collections::HashMap] with a String key and variable value
pub type Dictionary<T = String> = std::collections::HashMap<String, T>;
/// Type alias for [chrono::DateTime]
pub type ChronoDateTime<T = DefaultTimezone> = chrono::DateTime<T>;
/// Type alias for [config::ConfigBuilder]
pub type DefaultConfigBuilder = ConfigBuilder<DefaultConfigBuilderState>;
/// Type alias for [config::builder::DefaultState]
pub type DefaultConfigBuilderState = config::builder::DefaultState;
/// Type alias for the default timestamp
pub type DefaultTimestamp = i64;
/// Type alias for the default timezone, [chrono::Utc]
pub type DefaultTimezone = chrono::Utc;
/// Type alias for [std::io::Result]
pub type IOResult<T = ()> = std::io::Result<T>;

mod blocks {
    /// Type alias for a common block hash
    pub type BlockHs = String;
    /// Type alias for a common block id
    pub type BlockId = u64;
    /// Type alias for a common block nonce
    pub type BlockNc = u64;
    /// Type alias for a common block timestamp
    pub type BlockTs = super::DefaultTimestamp;
    /// Type alias for a common block timezone
    pub type BlockTz = super::DefaultTimezone;
    /// Type alias for a batch of transactions for a block as an array of type (T) and size 16
    pub type BlockTr<T = String> = [T; 16];
}