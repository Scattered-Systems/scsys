/*
    Appellation: agents <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{agent::*, specs::*};

pub(crate) mod agent;

pub(crate) mod specs {
    use serde::Serialize;

    pub trait Agency: Clone + Default + Serialize + ToString {
        fn init() -> Self;
        fn agent(&self) -> String;
    }
}
