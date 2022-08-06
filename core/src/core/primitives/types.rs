/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use blocks::*;
pub use collections::*;
pub use errors::*;
pub use misc::*;
pub use results::*;

mod blocks {
    /// Type alias for a common block hash
    pub type BlockHs = String;
    /// Type alias for a common block id
    pub type BlockId = u64;
    /// Type alias for a common block nonce
    pub type BlockNc = u64;
    /// Type alias for a common block timestamp
    pub type BlockTs = i64;
    /// Type alias for a common block timezone
    pub type BlockTz = chrono::Utc;
    /// Type alias for a batch of transactions for a block as an array of type (T) and size 16
    pub type BlockTr<T = String> = [T; 16];
}

mod collections {
    use super::ConfigFile;
    use config::{FileFormat, FileSourceFile};
    use std::collections::HashMap;

    /// Type alias for a vector ([Vec]) of config files ([crate::ConfigFile])
    pub type ConfigFileVec = Vec<ConfigFile<FileSourceFile, FileFormat>>;
    /// Type alias for an [std::collections::HashMap] with a String key and variable value
    pub type Dictionary<T = String> = HashMap<String, T>;
}

mod errors {
    /// Type alias for a boxed standard error
    pub type BaseError = Box<dyn std::error::Error>;
    /// Type alias for a boxed error with send, sync, and static flags enabled
    pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
}

mod misc {
    /// Type alias for [config::File]
    pub type ConfigFile<Src, Fmt> = config::File<Src, Fmt>;

    pub type ChronoDateTime<T = DefaultTimezone> = chrono::DateTime<T>;
    /// Type alias implementing a [ConfigBuilder] in its default state
    pub type DefaultConfigBuilder = crate::ConfigBuilder<crate::ConfigDefaultState>;
    /// Type alias for the default timestamp
    pub type DefaultTimestamp = i64;
    /// Type alias for the default timezone, [chrono::Utc]
    pub type DefaultTimezone = chrono::Utc;
}

mod results {
    use super::BoxError;

    /// Type alias of a result implementing the [super::BaseError]
    pub type BaseResult<T> = Result<T, super::BaseError>;
    /// Type alias for the standard result used
    pub type BoxResult<T> = Result<T, BoxError>;
    /// Type alias for [std::io::Result]
    pub type IOResult<T> = std::io::Result<T>;
}
