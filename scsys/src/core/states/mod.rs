/*
    Appellation: states <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{crud::*, power::*, state::*};

mod crud;
mod power;
mod state;

pub trait Stateful<Cnt>: Clone + PartialEq + std::fmt::Debug + std::hash::Hash {
    fn active(&self) -> bool;
    fn context(&self, state: String) -> Cnt;
    fn message(&self, message: String) -> String {
        format!("State (message:{:?}\n)", message)
    }
    fn timestamp(&self) -> crate::Timestamp {
        crate::Timestamp::default()
    }
}

