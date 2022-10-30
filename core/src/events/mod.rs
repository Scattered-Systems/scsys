/*
    Appellation: events <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{event::*, misc::*};

pub(crate) mod event;
pub(crate) mod misc;

pub trait Eventful {
    fn event(&self) -> &Self {
        self
    }
    fn id(&self) -> i64;
    fn timestamp(&self) -> i64;
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
