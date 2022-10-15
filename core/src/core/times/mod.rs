/*
    Appellation: scsys-core <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: Implements standard datetime utilities along with a timestamp structure for use throughout
*/
pub use self::timestamp::Timestamp;

mod timestamp;

pub(crate) mod primitives {}

pub(crate) mod utils {
    use chrono::{DateTime, TimeZone, Utc};

    pub fn chrono_datetime_now() -> DateTime<Utc> {
        Utc::now()
    }

    pub fn chrono_into_bson<T: TimeZone>(data: DateTime<T>) -> bson::DateTime {
        bson::DateTime::from_chrono(data)
    }
}
