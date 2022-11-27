/*
    Appellation: events <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{event::*, misc::*, specs::*};

pub(crate) mod event;
pub(crate) mod misc;

pub(crate) mod specs {
    use serde::Serialize;
    use std::fmt::Display;

    pub trait Eventful: Clone + Default + Display + Serialize {
        fn message(&self) -> String;
        fn timestamp(&self) -> i64;
    }
}

#[cfg(test)]
mod tests {
    use super::Event;

    #[test]
    fn test_event_default() {
        let a = Event::default();
        let b = a.clone();
        assert_eq!(a, b)
    }
}
