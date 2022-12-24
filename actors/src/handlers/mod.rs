/*
    Appellation: handlers <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{handler::*, specs::*, utils::*};

pub(crate) mod handler;

pub(crate) mod specs {
    use crate::states::{StatePack, Stateful};

    pub trait StatefulHandle<S: StatePack, T> {
        type State: Stateful<S, Data = T>;

        fn state(&self) -> &Self::State;
    }
}

pub(crate) mod utils {}
