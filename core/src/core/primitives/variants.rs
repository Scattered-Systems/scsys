/*
    Appellation: variants <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// A collection of time-related data structures
#[derive(Clone, Debug, PartialEq)]
pub enum Temporal {
    Bson(bson::DateTime),
    Datetime(DateTime<Utc>),
    Timestamp(i64),
}

impl Temporal {
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }
    pub fn bson_datetime() -> Self {
        Self::Bson(Self::now().into())
    }
    pub fn datetime() -> Self {
        Self::Datetime(DateTime::from(Self::now()))
    }
    pub fn timestamp() -> Self {
        Self::Timestamp(Self::now().timestamp())
    }
}

impl Default for Temporal {
    fn default() -> Self {
        Self::Timestamp(Self::now().timestamp())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub enum Id {
    Obj(bson::oid::ObjectId),
    Std(u64),
    Str(String),
}

impl Id {
    fn random_u64() -> u64 {
        let mut rnd = rand::thread_rng();
        rnd.gen::<u64>()
    }
    pub fn generate_object_id() -> Self {
        Self::Obj(bson::oid::ObjectId::new())
    }
    pub fn random_std() -> Self {
        Self::Std(Self::random_u64())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::Std(Self::random_u64())
    }
}

#[cfg(test)]
mod tests {
    use super::{Id, Temporal};

    #[test]
    fn test_ids() {
        let actual = Id::generate_object_id();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_temporal() {
        let actual = Temporal::default();
        let expected = actual.clone();
        assert_eq!(&actual, &expected)
    }
}
