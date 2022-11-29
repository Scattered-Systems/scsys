/*
    Appellation: contexts <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{context::*, specs::*};

pub(crate) mod context;

pub(crate) mod specs {
    use serde::Serialize;

    pub trait Configurable: Serialize {
        type Settings;

        fn settings(&self) -> &Self::Settings;
    }

    pub trait Contextual: ToString {
        type Cnf: Configurable;
        type Ctx;

        fn context(&self) -> &Self::Ctx;
    }
}
