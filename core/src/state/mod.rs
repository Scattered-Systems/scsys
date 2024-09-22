/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # State
//!
//! This module contains the stateful types and traits for the library.
#[doc(inline)]
pub use self::{interface::State, kinds::*};

mod interface;
mod kinds;

pub(crate) mod prelude {
    pub use super::interface::*;
    pub use super::kinds::*;
    pub use super::{RawState, Stateful};
}

/// [Stateful]
pub trait Stateful<T> {
    type State: RawState<Inner = T>;
}
/// [RawState]
pub trait RawState {
    type Inner;
    type Tag;
}

/*
 ************* Implementations *************
*/
impl<Q, T> RawState for State<Q, T> {
    type Inner = T;
    type Tag = Q;
}

impl<Q, T> Stateful<T> for State<Q, T> {
    type State = State<Q, T>;
}
