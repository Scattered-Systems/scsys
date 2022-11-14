/*
    Appellation: timestamp <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::{stamps::{chrono_datetime_now, chrono_into_bson}, ChronoDateTime};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub fn new(data: i64) -> Self {
        Self(data)
    }
    pub fn now() -> chrono::DateTime<Utc> {
        chrono_datetime_now()
    }
    pub fn chrono_to_bson(&self, data: ChronoDateTime) -> bson::DateTime {
        chrono_into_bson::<Utc>(data)
    }
    pub fn timestamp() -> i64 {
        Self::now().timestamp()
    }
}

impl std::convert::From<&Timestamp> for Timestamp {
    fn from(ts: &Timestamp) -> Self {
        Self(ts.0)
    }
}

impl std::convert::From<i64> for Timestamp {
    fn from(ts: i64) -> Self {
        Self(ts)
    }
}

impl std::convert::From<Timestamp> for i64 {
    fn from(ts: Timestamp) -> i64 {
        ts.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::new(Self::timestamp())
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let a = Timestamp::default();
        let b = a.clone();
        b.chrono_to_bson(Timestamp::now());
        assert_eq!(a, b)
    }

    #[test]
    fn test_timestamp_default() {
        let actual = Timestamp::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
