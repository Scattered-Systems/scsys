/*
    Appellation: temporal <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use chrono::{TimeZone, Utc};

pub trait Temporal {
    fn timestamp(&self) -> i64;
}

pub trait TemporalExt<Tz: TimeZone = Utc>: Temporal {
    fn now() -> chrono::DateTime<Tz>;
}
