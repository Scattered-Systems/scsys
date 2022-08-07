/*
    Appellation: generator <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StringGenerator {
    pub data: String,
    pub timestamp: i64,
}

impl StringGenerator {
    fn constructor(data: String, timestamp: i64) -> Self {
        Self { data, timestamp }
    }
    pub fn new() -> Self {
        Self::constructor(String::new(), crate::Temporal::now().timestamp())
    }
    pub fn generate(&self, len: usize) -> crate::BoxResult<Self> {
        crate::generate::generate_random_string(len);
        Ok(self.clone())
    }
}

impl Default for StringGenerator {
    fn default() -> Self {
        Self::new().generate(12).ok().unwrap()
    }
}
