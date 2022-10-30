/*
    Appellation: contexts <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::context::*;

pub(crate) mod context;

pub trait Configurable {
    fn settings(&self) -> &Self {
        self
    }
}

pub trait Contextual: Configurable {
    fn context(&self) -> &Self {
        self
    }
}
