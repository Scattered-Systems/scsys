/*
    Appellation: databases <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Database {
    pub name: String,
    pub uri: String,
}

impl Database {
    pub fn new(name: String, uri: String) -> Self {
        Self { name, uri }
    }
    pub fn from_str(name: &str, uri: &str) -> Self {
        Self::new(name.to_string(), uri.to_string())
    }
}
pub trait DatabaseSpec {
    fn name(&self) -> String;
    fn uri(&self) -> String;
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Databases {
    Cassandra { uri: String },
    MongoDB { name: String, uri: String },
    Postgres { name: String, uri: String },
    RocksDB { name: String, uri: String },
    SQLite(String),
}
