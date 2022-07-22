/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use constants::*;
pub use types::*;
pub use utils::*;

mod constants {
    const DIFFICULTY_PREFIX: &str = "00";
    const EPOCH: usize = 16;
}

mod types {
    pub use bson::oid::ObjectId;
    use hyper::server::conn::AddrIncoming;

    pub type AxumServer = axum::Server<AddrIncoming, axum::routing::IntoMakeService<axum::Router>>;

    /// Describes the type expected when considering a block id from a blockchain
    pub type BlockId = u64;
    /// Describes the type expected when considering a block nonce from a block on-chain.
    pub type BlockNc = u64;
    /// Describes the type expected when considering a block timestamp from a block on-chain.
    pub type BlockTs = i64;
    /// Defines the timezone implemented for each temporal ledger on-chain.
    pub type BlockTz = chrono::Utc;
    /// Describes the standard type to implement across the ecosystem
    pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
    /// A simplistic wrapper for the HashMap
    /// Assigns a <key>(string) -> <value>(Box<T>) where T defaults to a String
    pub type Dictionary<T = String> = std::collections::HashMap<String, Box<T>>;

    /// A collection of time-related data structures
    #[derive(Clone, Debug, Hash, PartialEq)]
    pub enum Clock<TimeZone: chrono::TimeZone = chrono::Utc> {
        Dt(bson::DateTime),
        Ts(i64),
        Tz(TimeZone),
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub enum Containers {
        KV(crate::KeyValue),
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub enum Id<T = String> {
        Obj(ObjectId),
        Other(T),
        Std(u64),
    }
}

mod utils {
    pub fn extractor<T>(string: String, breakpoint: char) -> Vec<T>
        where
            T: Clone + std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let exclude: &[char] = &[' ', ',', '[', ']', '.'];
        let trimmed: &str = &string.trim_matches(exclude);
        trimmed
            .split(breakpoint)
            .map(|i| i.trim_matches(exclude).parse::<T>().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor() {
        let a: Vec<u8> = extractor("0.0.0.0".to_string(), '.');
        let b: Vec<u8> = extractor("[0, 0, 0, 0]".to_string(), ',');
        assert_eq!(&a, &b)
    }
}
