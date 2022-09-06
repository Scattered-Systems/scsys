/*
    Appellation: time <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Timestamp(i64);

impl Timestamp {
    pub fn new(data: i64) -> Self {
        Self(data)
    }
    pub fn now() -> chrono::DateTime<chrono::Utc> {
        crate::DefaultTimezone::now()
    }
    pub fn chrono_to_bson(data: crate::ChronoDateTime) -> bson::DateTime {
        bson::DateTime::from_chrono(data)
    }
    pub fn timestamp() -> i64 {
        Self::now().timestamp()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::new(Self::timestamp())
    }
}

#[cfg(test)]
mod tests {
    use super::Timestamp;

    #[test]
    fn test_default_timestamp() {
        let actual = Timestamp::default();
        let expected = actual.clone();
        assert_eq!(&actual, &expected)
    }
}
