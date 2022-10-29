/*
    Appellation: loggers <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use super::logger_from_env;
use serde::{Deserialize, Serialize};
use tracing_subscriber;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Logger {
    pub level: String,
}

impl Logger {
    pub fn new(level: String) -> Self {
        Self { level }
    }

    pub fn from<T: std::string::ToString>(level: T) -> Self {
        Self::new(level.to_string())
    }

    pub fn setup(&self) {
        logger_from_env(Some(self.level.clone().as_str()))
    }
}

impl std::convert::From<&str> for Logger {
    fn from(level: &str) -> Self {
        Self::new(level.to_string())
    }
}
