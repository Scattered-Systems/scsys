/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # States
//!
//!
pub use self::state::State;

pub(crate) mod state;

/// [Stateful] describes a stateful object
pub trait Stateful {
    type State;

    /// [Stateful::state] is used to get the state of the object
    fn state(&self) -> &Self::State;
    /// [Stateful::update_state] is used to update the state of the object
    fn update_state(&mut self, state: Self::State);
}
