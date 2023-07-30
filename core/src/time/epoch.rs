/*
    Appellation: epoch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Epoch

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Epoch {
    duration: Duration,
    timestamp: i64,
}

impl Epoch {
    pub fn new(duration: Duration, timestamp: i64) -> Self {
        Self {
            duration,
            timestamp,
        }
    }
    pub fn duration(&self) -> Duration {
        self.duration
    }
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl From<Duration> for Epoch {
    fn from(duration: Duration) -> Self {
        Self::new(duration, chrono::Utc::now().timestamp())
    }
}