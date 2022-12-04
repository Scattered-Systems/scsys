/*
    Appellation: contexts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{context::*, specs::*};

pub(crate) mod context;

pub(crate) mod specs {
    use serde::Serialize;

    pub trait Configurable: Serialize {
        type Settings;

        fn settings(&self) -> &Self::Settings;
    }

    pub trait ConfigurableExt: Serialize {
        fn build() -> Result<Self, config::ConfigError>
        where
            Self: Sized;
    }

    pub trait Contextual: ToString {
        type Cnf: Configurable;
        type Ctx;

        fn context(&self) -> &Self::Ctx;
    }
}
