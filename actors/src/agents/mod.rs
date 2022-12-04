/*
    Appellation: agents <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{agent::*, specs::*};

pub(crate) mod agent;

pub(crate) mod specs {
    use crate::messages::Message;

    use serde::Serialize;
    use std::fmt::Display;

    pub trait AgentSpec<T: Display> {
        fn agent(&self) -> &Self;
        fn message(&self) -> &Message<T>;
    }

    pub trait Agency: Clone + Default + Serialize + ToString {
        fn init() -> Self;
        fn agent(&self) -> String;
    }
}
