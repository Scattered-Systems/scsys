/*
    Appellation: events <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{event::*, variants::*};

pub(crate) mod event;
mod variants;

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
