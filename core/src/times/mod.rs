/*
    Appellation: scsys-core <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: Implements standard datetime utilities along with a timestamp structure for use throughout
*/
pub use self::timestamp::Timestamp;

pub(crate) mod timestamp;

pub trait Temporal<Tz: chrono::TimeZone> {
    fn now(&self) -> Tz;
    fn timestamp(&self) -> i64;
}
