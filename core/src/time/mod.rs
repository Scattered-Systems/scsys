/*
    Appellation: time <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Time
pub use self::{epoch::*, timestamp::*};

mod epoch;
mod timestamp;

/// Interface for time-related data-structures
pub trait Temporal {
    fn timestamp(&self) -> i64;
}
