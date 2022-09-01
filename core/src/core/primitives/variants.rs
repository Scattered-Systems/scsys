/*
    Appellation: variants <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct Timestamp(i64);

impl Timestamp {
    fn constructor(data: i64) -> Self {
        Self(data)
    }
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

/// A collection of time-related data structures
#[derive(Clone, Debug, PartialEq)]
pub enum Temporal {
    Bson(bson::DateTime),
    Datetime(chrono::DateTime<chrono::Utc>),
    Timestamp(i64),
}

impl Temporal {
    pub fn now() -> crate::ChronoDateTime {
        chrono::Utc::now()
    }
    pub fn bson_datetime() -> Self {
        Self::Bson(Self::now().into())
    }
    pub fn datetime() -> Self {
        Self::Datetime(chrono::DateTime::from(Self::now()))
    }
    pub fn timestamp() -> Self {
        Self::Timestamp(Self::now().timestamp())
    }
}

impl Default for Temporal {
    fn default() -> Self {
        Self::timestamp()
    }
}

/// Captures the different ids allowed throughout the ecosystem
#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub enum Id {
    Num(u64),
    Obj(bson::oid::ObjectId),
    Str(String),
}

impl Id {
    pub fn new_nid() -> Self {
        Self::Num(crate::generate::generate_random_number::<u64>())
    }
    pub fn new_oid() -> Self {
        Self::Obj(bson::oid::ObjectId::new())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new_nid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_id() {
        let actual = Id::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_default_temporal() {
        let actual = Temporal::default();
        let expected = actual.clone();
        assert_eq!(&actual, &expected)
    }
}
