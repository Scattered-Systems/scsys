/*
    Appellation: specs <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{addressable::*, misc::*};

pub(crate) mod addressable;

pub(crate) mod misc {
    use crate::components::identities::Appellation;
    use chrono::{TimeZone, Utc};

    pub trait ActorSpec<T> {
        fn appellation(&self) -> Appellation<T>;
        fn justification(&self) -> serde_json::Value;
    }

    /// Interface for time-related data-structures
    pub trait Temporal {
        fn timestamp(&self) -> i64;
    }

    /// Interface extending the base features of Temporal
    pub trait TemporalExt<Tz: TimeZone = Utc>: Temporal {
        fn now() -> chrono::DateTime<Tz>;
    }
    pub trait Named {
        fn name() -> String;
    }
}
