/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{constants::*, types::*};

/// Type alias for [anyhow::Result]
pub type Result<T = ()> = std::result::Result<T, crate::errors::Error>;

mod constants {
    pub const SCASE: &str = "snake_case";
    ///
    pub const DEFAULT_HTTP_PORT: u16 = 8080;
    ///
    pub const DEFAULT_IGNORE_CHARS: &[char] = &['[', ']', ',', '.', ' '];
    /// Localhost in compatible array
    pub const LOCALHOST: [u8; 4] = [127, 0, 0, 1];
    /// Localhost as [&str]
    pub const LOCALHOST_STR: &str = "127.0.0.1";
}

mod types {
    pub use bson::oid::ObjectId;
    pub use config::{ConfigBuilder, ConfigError};
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    /// Type alias for async errors
    pub type AsyncError = Box<dyn std::error::Error + Send + Sync>;
    /// Type alias for async results
    pub type AsyncResult<T = ()> = std::result::Result<T, AsyncError>;
    /// Type alias for a boxed error with send, sync, and static flags enabled
    pub type BoxError = Box<dyn std::error::Error>;
    /// Type alias for the standard result used
    pub type BoxResult<T = (), E = BoxError> = std::result::Result<T, E>;
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
    /// Type alias wrapping a locked, thread-safe structure with a [std::sync::Mutex] in an [std::sync::Arc]
    pub type Locked<T> = Arc<Mutex<T>>;
    /// Type alias for [std::io::Result]
    pub type IOResult<T = ()> = std::io::Result<T>;

    /// Type alias for a stateful hash map
    pub type StateMap<Hs, V = (usize, usize)> = HashMap<Hs, V>;
}
