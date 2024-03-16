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
    type Timestamp;

    fn timestamp(&self) -> Self::Timestamp;
}

pub(crate) mod utils {

    /// [systime] is a utilitarian function that returns the current system time in milliseconds.
    pub fn systime() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_systime() {
        let start = systime();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let end = systime();
        assert!(end > start);
    }
}
