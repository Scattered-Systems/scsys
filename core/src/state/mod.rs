/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # State
//!
//! This module contains the stateful types and traits for the library.
#[doc(inline)]
pub use self::nstate::{NState, NStateKind};
/// this module implements an alternative stateful representation that enables one to provide
/// a data type as well as specify the state _kind_
pub mod nstate;

mod impls {
    pub mod impl_ops;
    pub mod impl_repr;
    pub mod impl_state;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::nstate::*;
    #[doc(inline)]
    pub use super::{RawState, Stateful};
}

/// [`State`] is a generic type wrapper materializing the [`RawState`] trait.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, transparent)
)]
#[repr(transparent)]
pub struct State<Q = usize>(pub Q);

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
