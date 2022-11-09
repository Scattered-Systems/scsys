/*
    Appellation: times <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{timestamp::Timestamp, utils::*};

pub(crate) mod timestamp;

pub trait Temporal<Tz: chrono::TimeZone = chrono::Utc> {
    fn now(&self) -> chrono::DateTime<Tz>;
    fn timestamp(&self) -> i64;
}

pub(crate) mod utils {
    use chrono::{DateTime, TimeZone, Utc};

    pub fn chrono_datetime_now() -> DateTime<Utc> {
        Utc::now()
    }

    pub fn chrono_into_bson<T: TimeZone>(data: DateTime<T>) -> bson::DateTime {
        bson::DateTime::from_chrono(data)
    }

    pub fn timestamp() -> i64 {
        Utc::now().timestamp()
    }
}
