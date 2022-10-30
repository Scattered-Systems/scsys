/*
    Appellation: time <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

use chrono::{DateTime, TimeZone, Utc};

pub fn chrono_datetime_now() -> DateTime<Utc> {
    Utc::now()
}

pub fn chrono_into_bson<T: TimeZone>(data: DateTime<T>) -> bson::DateTime {
    bson::DateTime::from_chrono(data)
}
