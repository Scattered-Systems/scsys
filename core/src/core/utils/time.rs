/*
    Appellation: time <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub fn chrono_datetime_now() -> DateTime<Utc> {
    Utc::now()
}

pub fn chrono_into_bson<T: chrono::TimeZone>(data: DateTime<T>) -> bson::DateTime {
    bson::DateTime::from_chrono(data)
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Timestamp(i64);

impl Timestamp {
    pub fn new(data: i64) -> Self {
        Self(data)
    }
    pub fn now() -> DateTime<Utc> {
        chrono_datetime_now()
    }
    pub fn chrono_to_bson(&self, data: crate::ChronoDateTime) -> bson::DateTime {
        chrono_into_bson::<Utc>(data)
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
