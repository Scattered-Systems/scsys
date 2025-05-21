/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Identity
//!
//! The identity module provides a set of traits and types for generating unique identifiers.
#[doc(inline)]
pub use self::{identifier::*, kinds::prelude::*, traits::*};

pub(crate) mod identifier;
pub(crate) mod traits;

pub mod kinds {
    #[doc(inline)]
    pub use self::indexed::IndexId;

    pub mod indexed;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::indexed::IndexId;
    }
}

pub(crate) mod prelude {
    pub use super::identifier::Id;
    pub use super::kinds::prelude::*;
    pub use super::traits::*;
}

#[cfg(test)]
mod tests {
    use super::Id;
    use super::traits::Identity;

    #[test]
    fn test_id() {
        let id = 0usize.get();
        assert_eq!(id, &0);
        let atomic = Id::atomic();
        let aid = Id::<usize>::get(&atomic);
        assert_ne!(*aid, *id);
    }
}
