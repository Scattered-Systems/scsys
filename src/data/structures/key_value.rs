/*
    Appellation: key_value <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct KeyValue {
    pub key: String,
    pub value: Vec<String>,
}

impl KeyValue {
    fn constructor(key: String, value: Vec<String>) -> Result<Self, crate::BoxError> {
        Ok(Self { key, value })
    }
    pub fn new(key: String, value: Vec<String>) -> Self {
        match Self::constructor(key, value) {
            Ok(v) => v,
            Err(e) => panic!("KeyValue Error: {}", e),
        }
    }
    pub fn from(key: &str, value: Vec<&str>) -> Self {
        let mut val = Vec::<String>::new();
        for i in 0..value.clone().len() {
            val.push(String::from(value[i]))
        }
        Self::new(String::from(key), val)
    }
}

impl std::fmt::Display for KeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KeyValue(\n\tkey={},\n\tvalue={:#?}\n)",
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
