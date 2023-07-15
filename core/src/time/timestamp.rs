/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Timestamp
use super::Temporal;
use crate::{BsonDateTime, ChronoDateTime, DefaultTimezone};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumVariantNames};

/// Timestamp implements a host of useful utilities for stamping data
#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Timestamp {
    Datetime(bson::DateTime),
    Rfc3339(String),
    Ts(i64),
}

impl Timestamp {
    pub fn new() -> Self {
        Self::Ts(DefaultTimezone::now().timestamp())
    }
    pub fn now() -> ChronoDateTime {
        chrono::Utc::now()
    }
    pub fn chrono_to_bson(&self, data: ChronoDateTime) -> bson::DateTime {
        bson::DateTime::from_chrono(data)
    }
    pub fn bson_datetime() -> bson::DateTime {
        bson::DateTime::from_chrono(Self::now())
    }
    pub fn datetime() -> ChronoDateTime {
        chrono::Utc::now()
    }
    pub fn gen_rfc3339() -> String {
        chrono::Utc::now().to_rfc3339()
    }
    pub fn rfc3339() -> Self {
        Self::Rfc3339(chrono::Utc::now().to_rfc3339())
    }
}

impl Temporal for Timestamp {
    fn timestamp(&self) -> i64 {
        match self.clone() {
            Self::Rfc3339(ts) => chrono::DateTime::parse_from_rfc3339(ts.as_str())
                .unwrap()
                .timestamp(),
            Self::Ts(ts) => ts,
            Timestamp::Datetime(ts) => ts.to_chrono().timestamp(),
        }
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::Rfc3339(chrono::Utc::now().to_rfc3339())
    }
}

impl From<i64> for Timestamp {
    fn from(data: i64) -> Self {
        Self::Ts(data)
    }
}

impl From<BsonDateTime> for Timestamp {
    fn from(ts: BsonDateTime) -> Self {
        Self::Datetime(ts)
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Timestamp {
    fn from(ts: chrono::DateTime<chrono::Utc>) -> Self {
        Self::Rfc3339(ts.to_rfc3339())
    }
}

impl TryFrom<String> for Timestamp {
    type Error = crate::BoxError;

    fn try_from(data: String) -> Result<Self, Self::Error> {
        Self::try_from(data.as_str())
    }
}

impl TryFrom<&str> for Timestamp {
    type Error = crate::BoxError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let dt = chrono::DateTime::parse_from_rfc3339(value)?;
        Ok(Self::Ts(dt.timestamp()))
    }
}

impl From<Timestamp> for i64 {
    fn from(data: Timestamp) -> i64 {
        data.timestamp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let boundary = Timestamp::now();
        let a = Timestamp::from(boundary.clone());
        let b = Timestamp::from(boundary.timestamp());
        assert_ne!(a, b);
        assert_eq!(a.timestamp(), b.timestamp());
    }

    #[test]
    fn test_timestamp_default() {
        let m = Timestamp::default();
        let a = m.clone();
        let b: i64 = m.clone().into();
        assert_eq!(&m, &a);
        assert_eq!(&a.timestamp(), &b);
    }
}
