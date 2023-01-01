/*
    Appellation: loggers <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Loggable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Logger {
    pub level: String,
}

impl Logger {
    pub fn new(level: String) -> Self {
        Self { level }
    }
    pub fn setup(&mut self, key: Option<&str>) -> &Self {
        let key = key.unwrap_or("RUST_LOG");
        if let Some(v) = std::env::var_os(key) {
            self.level = v.into_string().expect("Failed to convert into string...");
        } else {
            std::env::set_var(key, self.level.clone());
        }
        self
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::from("info")
    }
}

impl Loggable for Logger {
    fn level(&self) -> String {
        self.level.clone()
    }
}

impl std::convert::From<&Logger> for Logger {
    fn from(data: &Logger) -> Self {
        Self::new(data.level.clone())
    }
}

impl std::convert::From<&str> for Logger {
    fn from(level: &str) -> Self {
        Self::new(level.to_string())
    }
}
