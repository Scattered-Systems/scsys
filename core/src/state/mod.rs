/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # State
//!
//! This module contains the stateful types and traits for the library.
#[doc(inline)]
pub use self::nstate::*;

pub mod nstate;

pub(crate) mod prelude {
    pub use super::nstate::*;
    pub use super::{RawState, Stateful};
}

/// a trait for denoting stateful entities
pub trait Stateful {
    type State: RawState;
}

/// [RawState]
pub trait RawState {
    type Inner;
}

/*
 ************* Implementations *************
*/
impl<Q, T> RawState for State<Q, T> {
    type Inner = T;
}

impl<Q, T> Stateful for State<Q, T> {
    type State = State<Q, T>;
}
