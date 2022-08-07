/*
    Appellation: alphabet <module>
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
    pub fn new(data: String) -> Self {
        Self::constructor(data, crate::Temporal::now().timestamp())
    }
    pub fn generate(len: usize) -> crate::BoxResult<Self> {
        Ok(Self::new(crate::generate::generate_random_string(len)))
    }
}

impl Default for StringGenerator {
    fn default() -> Self {
        Self::generate(12).ok().unwrap()
    }
}
