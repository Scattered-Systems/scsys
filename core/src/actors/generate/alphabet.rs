/*
    Appellation: alphabet <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{generate::generate_random_string, Timestamp};

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StringGenerator {
    pub data: String,
    pub timestamp: Timestamp,
}

impl StringGenerator {
    pub fn new(len: usize) -> Self {
        let data = generate_random_string(len);
        let timestamp = Timestamp::default();
        Self { data, timestamp }
    }
}

impl Default for StringGenerator {
    fn default() -> Self {
        Self::new(12)
    }
}

#[cfg(test)]
mod tests {
    use super::StringGenerator;

    #[test]
    fn test_string_generator_default() {
        assert_ne!(StringGenerator::default(), StringGenerator::default())
    }
}
