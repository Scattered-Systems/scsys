/*
    Appellation: agents <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{agent::*, contexts::*, specs::*, states::*};

pub(crate) mod agent;
pub(crate) mod contexts;
pub(crate) mod states;

pub(crate) mod specs {
    use serde::Serialize;

    pub trait Agency: Clone + Default + Serialize + ToString {
        fn init() -> Self;
        fn agent(&self) -> String;
    }
}
