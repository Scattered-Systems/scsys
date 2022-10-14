/*
    Appellation: interface <events>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/


pub trait EventSpec {
    type Message;

    fn message(&self) -> Self::Message;
    fn timestamp(&self) -> crate::Timestamp;
}
