/*
    Appellation: events <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{payload::*, variants::*};

pub(crate) mod payload;
pub(crate) mod variants;

pub trait Eventful {
    fn event(&self) -> &Self {
        self
    }
    fn id(&self) -> i64;
    fn timestamp(&self) -> i64;
}
