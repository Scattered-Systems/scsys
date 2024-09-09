/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # State
//!
//! This module contains the stateful types and traits for the library.
#[doc(inline)]
pub use self::{base_state::*, kinds::*};

mod base_state;
mod kinds;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::base_state::*;
    pub use super::kinds::*;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nary_state() {
        let state = NState::<usize, 4>::new(0);
        assert!(state.is_state::<Nary<4>>());

        assert!(!state.is_state::<Nary<2>>());
        assert!(!state.is_state::<Nary<{ usize::MAX }>>());
    }
}
