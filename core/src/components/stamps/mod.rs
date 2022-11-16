/*
    Appellation: times <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{specs::*, timestamp::Timestamp, utils::*};

pub(crate) mod timestamp;

pub(crate) mod specs {
    use chrono::{TimeZone, Utc};

    /// Interface for time-related data-structures
    pub trait Temporal {
        fn timestamp(&self) -> i64;
    }

    /// Interface extending the base features of Temporal
    pub trait TemporalExt<Tz: TimeZone = Utc>: Temporal {
            fn now() -> chrono::DateTime<Tz>;
        }

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
