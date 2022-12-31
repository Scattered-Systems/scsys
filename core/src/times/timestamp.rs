/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{timestamp, DefaultTimezone, Temporal, TemporalExt};
use serde::{Deserialize, Serialize};
use std::convert::{From, TryFrom};
use strum::EnumVariantNames;

/// Timestamp implements a host of useful utilities for stamping data
#[derive(Clone, Debug, Deserialize, EnumVariantNames, Eq, Hash, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum Timestamp {
    Rfc3339(String),
    Ts(i64),
}

impl Timestamp {
    pub fn new() -> Self {
        Self::Ts(timestamp())
    }
    pub fn now() -> chrono::DateTime<DefaultTimezone> {
        chrono::Utc::now()
    }
    pub fn rfc3339() -> Self {
        Self::Rfc3339(chrono::Utc::now().to_rfc3339())
    }
    pub fn timestamp(&self) -> i64 {
        self.into()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::Ts(timestamp())
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
impl Temporal for Timestamp {
    fn created(&self) -> i64 {
        self.into()
    }
}
impl TemporalExt for Timestamp {}

impl From<&Timestamp> for Timestamp {
    fn from(data: &Timestamp) -> Self {
        data.clone()
    }
}

impl From<&chrono::DateTime<chrono::Utc>> for Timestamp {
    fn from(ts: &chrono::DateTime<chrono::Utc>) -> Self {
        Self::Ts(ts.timestamp())
    }
}

impl From<i64> for Timestamp {
    fn from(data: i64) -> Self {
        Self::Ts(data)
    }
}

impl From<Timestamp> for i64 {
    fn from(data: Timestamp) -> i64 {
        match data {
            Timestamp::Rfc3339(ts) => chrono::DateTime::parse_from_rfc3339(ts.as_str())
                .unwrap()
                .timestamp(),
            Timestamp::Ts(ts) => ts,
        }
    }
}

impl From<&Timestamp> for i64 {
    fn from(data: &Timestamp) -> i64 {
        data.clone().into()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let ts = Timestamp::now();
        let str_ts = ts.to_rfc3339();
        let a = Timestamp::from(&ts);
        let b = Timestamp::try_from(str_ts).unwrap();
        assert_eq!(a, b)
    }

    #[test]
    fn test_timestamp_default() {
        let m = Timestamp::default();
        let a = Timestamp::from(&m);
        let b: i64 = Timestamp::from(&a).into();
        assert_eq!(&m, &a);
        assert_eq!(&a.timestamp(), &b);
    }
}
