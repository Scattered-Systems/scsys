/*
    Appellation: types <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use std::collections::HashMap;

pub use config::{AsyncConfigBuilder, ConfigBuilder, ConfigError};

pub type AsyncError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type AsyncResult<T = ()> = Result<T, AsyncError>;
/// Type alias for a boxed standard error
pub type BaseError = Box<dyn std::error::Error>;
/// Type alias for a boxed error with send, sync, and static flags enabled
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Type alias of a result implementing the [BaseError]
pub type Result<T = (), E = BaseError> = std::result::Result<T, E>;
/// Type alias for the standard result used
pub type BoxResult<T = (), E = BoxError> = Result<T, E>;
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
pub type ConfigResult<T> = Result<T, config::ConfigError>;
/// Type alias for [std::collections::HashMap] defaulting to a (String, String) type
pub type Dictionary<K = String, V = String> = HashMap<K, V>;
/// Type alias for [chrono::DateTime]
pub type ChronoDateTime<T = DefaultTimezone> = chrono::DateTime<T>;
/// Type alias for [config::ConfigBuilder]
pub type DefaultConfigBuilder = super::ConfigBuilder<DefaultConfigBuilderState>;
/// Type alias for [config::builder::DefaultState]
pub type DefaultConfigBuilderState = config::builder::DefaultState;
/// Type alias for the default timestamp
pub type DefaultTimestamp = i64;
/// Type alias for the default timezone, [chrono::Utc]
pub type DefaultTimezone = chrono::Utc;
/// Type alias for [std::io::Result]
pub type IOResult<T = ()> = std::io::Result<T>;

/// Type alias for a stateful hash map
pub type StateMap<Hs, V = (usize, usize)> = HashMap<Hs, V>;
