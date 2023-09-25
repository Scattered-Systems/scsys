/*
    Appellation: time <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Time
pub use self::{datetime::*, epoch::*, timestamp::*, utils::*};

pub(crate) mod datetime;
pub(crate) mod epoch;
pub(crate) mod timestamp;

/// Interface for time-related data-structures
pub trait Temporal {
    fn timestamp(&self) -> i64;
}

pub(crate) mod utils {

    pub fn system_timestamp() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}
