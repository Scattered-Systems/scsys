/*
    Appellation: errors <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Error {
    AsyncError,
    ConnectionError,
    Default,
}

impl Error {
    pub fn new(data: &str) -> Self {
        Self::metadata()
            .get(data)
            .expect("Failed to find a match...")
            .clone()
    }
    pub fn metadata() -> crate::Dictionary<Self> {
        let options = [
            ("async".to_string(), Self::AsyncError),
            ("connection".to_string(), Self::ConnectionError),
            ("default".to_string(), Self::Default),
        ];
        crate::Dictionary::from(options)
    }
}

impl Default for Error {
    fn default() -> Self {
        Self::new("default")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_errors() {
        let actual = Error::default();
        let expected = Error::Default;
        assert_eq!(actual, expected)
    }
}
