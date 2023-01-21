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

        fn by_ref(&self) -> &Self {
            self
        }
        fn by_mut_ref(&self) -> &Self {
            self
        }
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

        fn by_ref(&self) -> &Self {
            self
        }
        fn by_mut_ref(&self) -> &Self {
            self
        }
        fn context(&self) -> &Self::Ctx;
    }
}
