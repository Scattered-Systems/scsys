/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # States
//!
//! This modules works to implement a host of primitives and utilities for working with stateful objects.
#[doc(inline)]
pub use self::state::*;

pub(crate) mod state;

pub mod kinds;

pub(crate) mod prelude {
    pub use super::state::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let state = State::binary(true);

        assert!(*state.get());
    }
}
