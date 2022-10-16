/*
    Appellation: database <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description: Implements the standards for interacting with database providers
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

impl std::convert::From<&Self> for Database {
    fn from(data: &Self) -> Self {
        data.clone()
    }
}

impl std::convert::From<(&str, &str)> for Database {
    fn from(data: (&str, &str)) -> Self {
        Self::new(data.0.to_string(), data.1.to_string())
    }
}

impl std::convert::From<&str> for Database {
    fn from(data: &str) -> Self {
        Self::new(String::new(), data.to_string())
    }
}
