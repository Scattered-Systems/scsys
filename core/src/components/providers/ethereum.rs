/*
    Appellation: providers <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Web3Provider {
    pub name: String,
    pub url: String,
}

impl Web3Provider {
    pub fn new(name: String, url: String) -> Self {
        Self { name, url }
    }
    pub fn from_str(name: &str, url: &str) -> Self {
        Self::new(name.to_string(), url.to_string())
    }
}

impl Default for Web3Provider {
    fn default() -> Self {
        Self::from_str("localhost", "http://localhost:8545")
    }
}
