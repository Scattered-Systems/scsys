/*
    Appellation: handlers <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{handler::*, specs::*, utils::*};

pub(crate) mod handler;

pub(crate) mod specs {
    use crate::states::Stateful;

    pub trait StateHandle<S: Stateful> {
        fn state(&self) -> &S;
    }
}

pub(crate) mod utils {}
