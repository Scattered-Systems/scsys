/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # State
//!
//! This module contains the stateful types and traits for the library.
#[doc(inline)]
pub use self::{
    nstate::{NState, NStateKind},
    wrapper::State,
};
/// this module implements an alternative stateful representation that enables one to provide
/// a data type as well as specify the state _kind_
pub mod nstate;
pub mod wrapper;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::nstate::*;
    #[doc(inline)]
    pub use super::wrapper::*;
    #[doc(inline)]
    pub use super::{RawState, StateBase, Stateful};
}

/// [`Stateful`] is a trait establishing a common interface for all types that are state-aware.
pub trait Stateful<Q> {
    type State<Q2>: StateBase<Item = Q2>;

    fn state(&self) -> &Self::State<Q>;
}

/// [RawState] is a trait that defines the types of states
pub trait RawState {
    private!();
}

/// The [`StateBase`] trait defines the base type for stateful items, allowing for a generic
/// item type to be associated with the state.
pub trait StateBase {
    type Item;

    private!();
}

/*
 ************* Implementations *************
*/
impl<Q> StateBase for State<Q> {
    type Item = Q;

    seal!();
}

impl<Q, T> StateBase for NState<Q, T> {
    type Item = T;

    seal!();
}
