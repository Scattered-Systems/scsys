/*
    Appellation: databases <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description: Implements the standards for interacting with database providers
*/
pub use self::{database::Database, interface::DatabaseSpec, variants::Databases};

pub(crate) mod database;

pub(crate) mod interface {
    pub trait DatabaseSpec {
        fn name(&self) -> String;
        fn uri(&self) -> String;
    }
}

pub(crate) mod variants {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub enum Databases {
        Cassandra { uri: String },
        MongoDB { name: String, uri: String },
        Postgres { name: String, uri: String },
        RocksDB { name: String, uri: String },
        SQLite(String),
    }
}
