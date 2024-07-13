/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # States
//!
//! This modules works to implement a host of primitives and utilities for working with stateful objects.
pub use self::{kinds::prelude::*, state::StateBase, traits::*};

pub(crate) mod state;
pub(crate) mod traits;

pub mod kinds {
    pub use self::binary::{BinState, BinaryState};

    pub mod binary;

    pub(crate) mod prelude {
        pub use super::binary::{BinState, BinaryState};
    }
}

pub(crate) mod prelude {
    pub use super::kinds::prelude::*;
    pub use super::state::StateBase;
    pub use super::traits::*;
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_states() {
        let data = "sample";
        let a = BinState::valid(data);
        let b = BinState::invalid(data);
        assert_ne!(a, b);
        assert_eq!(a.as_ref(), b.as_ref());
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_states_iter() {
        let a: Vec<BinaryState> = BinaryState::iter().collect();
        assert_eq!(a.len(), 2);
        assert_eq!(a[0], BinaryState::invalid());
    }
}
