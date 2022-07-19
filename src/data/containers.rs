/*
    Appellation: containers <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Containers {
    KV(KeyValue),
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl KeyValue {
    fn constructor(key: String, value: String) -> Result<Self, crate::BoxError> {
        Ok(Self { key, value })
    }
    pub fn new(key: String, value: String) -> Self {
        Self::constructor(key, value).ok().unwrap()
    }
    pub fn from(key: &str, value: &str) -> Self {
        Self::constructor(String::from(key), String::from(value))
            .ok()
            .unwrap()
    }
}

impl std::fmt::Display for KeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KeyValue(\n\tkey={},\n\tvalue={}\n)",
            self.key, self.value
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
