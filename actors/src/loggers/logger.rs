/*
    Appellation: loggers <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::specs::Loggable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Logger {
    pub level: String,
}

impl Logger {
    pub fn new(level: String) -> Self {
        Self { level }
    }
    pub fn set_level(mut self, level: impl ToString) {
        self.level = level.to_string();
    }
    pub fn setup_env(mut self, level: Option<&str>) -> Self {
        let key = level.unwrap_or("RUST_LOG");
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
        Self::new("info".to_string())
    }
}

impl Loggable for Logger {
    fn level(&self) -> String {
        self.level.clone()
    }
}

impl<T> From<&T> for Logger
where
    T: ToString,
{
    fn from(level: &T) -> Self {
        Self::new(level.to_string())
    }
}
