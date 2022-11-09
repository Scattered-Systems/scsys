/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub trait Eventful {
    type Event;
    fn event(&self) -> &Self::Event;
    fn id(&self) -> i64;
    fn timestamp(&self) -> i64;
}
