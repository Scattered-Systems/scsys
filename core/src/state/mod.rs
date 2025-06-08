/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! this module implements a generic [`State`] wrapper and provides several traits to support
//! state management and other stateful workloads.
//!
#[doc(inline)]
pub use self::{
    nstate::{NState, NStateKind},
    traits::prelude::*,
    wrapper::State,
};
/// this module implements an alternative stateful representation that enables one to provide
/// a data type as well as specify the state _kind_
pub mod nstate;
pub mod wrapper;

mod impls {
    pub mod impl_wrapper;
    pub mod impl_wrapper_ops;
}

pub mod traits {
    //! this module implements various traits supporting the [`Id`](super::Id) type
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod state;
    pub mod state_repr;
    pub mod stateful;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::state::*;
        #[doc(inline)]
        pub use super::state_repr::*;
        #[doc(inline)]
        pub use super::stateful::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::nstate::*;
    #[doc(inline)]
    pub use super::traits::prelude::*;
    #[doc(inline)]
    pub use super::wrapper::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let mut state = State::<usize>::zero();
        // verify the initial state is zero
        assert_eq!(state, 0);
        // set the state to a new value
        state.set(5);
        // verify the state is now 5
        assert_eq!(state, 5);
        // take the inner value leaving the default in its place
        assert_eq!(state.take(), 5);
        // verify the state is now back to its default value
        assert_eq!(state, 0);
        // map the state to a new value
        let mapped = state.map(|x| x + 1);
        assert_eq!(mapped, 1);
        // ensure the original state is unchanged
        assert_ne!(mapped, state);
    }

    #[test]
    fn test_state_views() {
        let mut state = State::<usize>::zero();

        let view = state.view();
        // verify that the "view" contains a reference to the original value
        assert_eq!(state.get(), view.value());
        // set the state to a new value
        state.set(5);
        // verify that the "view_mut" contains a mutable reference to the original value
        assert_eq!(state.view().value(), &mut 5_usize);
    }

    #[cfg(feature = "rand")]
    #[test]
    fn test_state_rand() {
        // generate some random state `a`
        let a = State::<f32>::random();
        // generate another random state `b`
        let b = State::<f32>::random();
        // ensure the two states are not equal
        assert_ne!(a, b);
    }
}
