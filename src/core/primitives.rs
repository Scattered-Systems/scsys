/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use constants::*;
pub use types::*;

mod constants {
    const DIFFICULTY_PREFIX: &str = "00";
    const EPOCH: usize = 16;
}

mod types {
    pub use bson::oid::ObjectId;
    pub use chrono::{DateTime, Utc};

    /// Outlines the expected type for a block hash
    pub type BlockHs = String;
    /// Describes the type expected when considering a block id from a blockchain
    pub type BlockId = u64;
    /// Describes the type expected when considering a block nonce from a block on-chain.
    pub type BlockNc = u64;
    /// Describes the type expected when considering a block timestamp from a block on-chain.
    pub type BlockTs = i64;
    /// Defines the timezone implemented for each temporal ledger on-chain.
    pub type BlockTz = chrono::Utc;
    /// Defines a block of transactions to be an array of 16 transactions <type> (T)
    pub type BlockTr<T = String> = [T; 16];
    /// Defines a dynamic standard error with send, sync, and static features
    pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
    /// Describes a configuration builder in their default state
    pub type DefaultConfigBuilder = config::ConfigBuilder<config::builder::DefaultState>;
    /// A simplistic wrapper for the HashMap
    /// Assigns a <key>(string) -> <value>(Box<T>) where T defaults to a String
    pub type Dictionary<T = String> = std::collections::HashMap<String, Box<T>>;
    // Describes a boxed dynamic error
    pub type StdError = Box<dyn std::error::Error>;
    /// Describes the result of a collection of configuration files
    pub type ConfigFileVec = Vec<config::File<config::FileSourceFile, config::FileFormat>>;

    /// A collection of time-related data structures
    #[derive(Clone, Debug, Hash, PartialEq)]
    pub enum Clock {
        Dt(bson::DateTime),
        Ts(i64),
        Tz(DateTime<Utc>),
    }

    impl Clock {
        fn now() -> DateTime<Utc> {
            Utc::now()
        }
        pub fn bson_datetime() -> Self {
            Self::Dt(Self::now().into())
        }
        pub fn datetime() -> Self {
            Self::Tz(Self::now())
        }
        pub fn timestamp() -> Self {
            Self::Ts(Self::now().timestamp())
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub enum Id<T = String> {
        Obj(ObjectId),
        Other(T),
        Std(u64),
    }

    impl Id {
        pub fn generate_object_id() -> Self {
            Self::Obj(ObjectId::new())
        }
    }
}
