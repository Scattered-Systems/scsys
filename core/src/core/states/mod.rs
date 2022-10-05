/*
    Appellation: states <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{crud::CRUDState, power::PowerState, state::State};

mod crud;
mod power;
mod state;

pub trait Stateful<S, T> {
    fn state(&self) -> S
    where
        Self: Sized;
    fn transition(&self, state: S, f: dyn Fn(S) -> T) -> T;
}
