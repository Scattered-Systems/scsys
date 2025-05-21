/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # State
//!
//! This module contains the stateful types and traits for the library.
#[doc(inline)]
pub use self::{nstate::*, state::*};

pub mod nstate;
pub mod state;

mod impls {
    pub mod impl_ops;
    pub mod impl_repr;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::nstate::*;
    #[doc(inline)]
    pub use super::state::*;
    #[doc(inline)]
    pub use super::{RawState, Stateful};
}

/// a trait for denoting stateful entities
pub trait Stateful {
    type State: RawState;
}

/// [RawState]
pub trait RawState {
    type Item;

    private!();
}

/*
 ************* Implementations *************
*/
impl<Q, T> RawState for NState<Q, T> {
    type Item = T;

    seal!();
}

impl<Q, T> Stateful for NState<Q, T> {
    type State = NState<Q, T>;
}
