/*
    Appellation: alphabet <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::generate_random_string;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct StringGenerator {
    pub data: String,
    pub timestamp: i64,
}

impl StringGenerator {
    pub fn new(len: usize) -> Self {
        let data = generate_random_string(len);
        let timestamp = chrono::Utc::now().timestamp();
        Self { data, timestamp }
    }
}

impl Default for StringGenerator {
    fn default() -> Self {
        Self::new(12)
    }
}
