/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{chrono_into_bson, ChronoDateTime};
use chrono::Utc;

pub trait Addressable {
    type Addr;
    fn address(self) -> Self::Addr;
}
/// Trait for signaling a structure with a dedicated build stage
pub trait Buildable {
    type Error;
    fn build() -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// Quickly derive elligible naming schematics for the desired structure
pub trait Named {
    fn name(&self) -> String;
    fn slug(&self) -> String {
        self.name().to_lowercase()
    }
}

pub trait TemporalExt: Temporal {
    fn chrono_to_bson(&self, data: ChronoDateTime) -> bson::DateTime {
        chrono_into_bson::<Utc>(data)
    }
    fn bson_datetime() -> bson::DateTime {
        chrono_into_bson::<chrono::Utc>(chrono::Utc::now())
    }
    fn datetime() -> ChronoDateTime {
        chrono::Utc::now()
    }
    fn gen_rfc3339() -> String {
        chrono::Utc::now().to_rfc3339()
    }
    fn ts() -> i64 {
        chrono::Utc::now().timestamp()
    }
}

/// Interface for time-related data-structures
pub trait Temporal {
    fn created(&self) -> i64; // Recall an objects time of creation
}
