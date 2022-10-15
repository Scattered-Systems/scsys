/*
    Appellation: interface <events>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::Timestamp;

pub trait EventSpec {
    type Event;

    fn event(&self) -> Self::Event;
    fn timestamp(&self) -> Timestamp;
}
