/*
    Appellation: timestamp <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::{chrono_datetime_now, chrono_into_bson};
use chrono::Utc;
use serde::{Deserialize, Serialize};

pub trait Temporal {
    fn now(&self) -> Timestamp {
        Timestamp::default()
    }
    fn timestamp(&self) -> Timestamp;

}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Timestamp(i64);

impl Timestamp {
    pub fn new(data: i64) -> Self {
        Self(data)
    }
    pub fn now() -> chrono::DateTime<Utc> {
        chrono_datetime_now()
    }
    pub fn chrono_to_bson(&self, data: crate::ChronoDateTime) -> bson::DateTime {
        chrono_into_bson::<Utc>(data)
    }
    pub fn timestamp() -> i64 {
        Self::now().timestamp()
    }
}

impl std::convert::Into<i64> for Timestamp {
    fn into(self) -> i64 {
        self.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::new(Self::timestamp())
    }
}
