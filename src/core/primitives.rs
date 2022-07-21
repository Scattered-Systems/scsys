/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use utils::*;

use hyper::server::conn::AddrIncoming;

pub type AxumServer = axum::Server<AddrIncoming, axum::routing::IntoMakeService<axum::Router>>;
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Dictionary<T = String> = std::collections::HashMap<String, Box<T>>;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Id<T = String> {
    Obj(bson::oid::ObjectId),
    Other(T),
    Std(u64),
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Containers {
    KV(crate::KeyValue),
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
