/*
    Appellation: loggers <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use super::{logger_from_env, Loggable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Logger {
    pub level: String,
}

impl Logger {
    pub fn new(level: String) -> Self {
        Self { level }
    }
    pub fn setup(&self) {
        logger_from_env(Some(self.level.clone().as_str()))
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

impl std::convert::From<&str> for Logger {
    fn from(level: &str) -> Self {
        Self::new(level.to_string())
    }
}
