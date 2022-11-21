/*
    Appellation: specs <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{addressable::*, misc::*};

pub(crate) mod addressable;

pub(crate) mod misc {
    use crate::{chrono_into_bson, Appellation, ChronoDateTime};
    use chrono::Utc;
    use serde_json::Value;

    

    pub trait InputName {
        fn name(&self) -> String;
        fn slug(&self) -> String {
            self.name().to_lowercase()
        }
    }

    pub trait ActorSpec<T> {
        fn appellation(&self) -> Appellation<T>;
        fn justification(&self) -> serde_json::Value;
    }

    /// Interface for time-related data-structures
    pub trait Temporal {
        fn chrono_to_bson(&self, data: ChronoDateTime) -> bson::DateTime {
            chrono_into_bson::<Utc>(data)
        }
        fn datetime() -> ChronoDateTime {
            chrono::Utc::now()
        }
        fn timestamp(&self) -> i64; // Recall an objects time of creation
    }

    /// Quickly derive elligible naming schematics for the desired structure
    pub trait Named {
        fn name() -> String;
        fn slug(&self) -> String {
            Self::name().clone().to_lowercase()
        }
    }
}
