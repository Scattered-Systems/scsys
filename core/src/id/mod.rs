/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Identity
//!
//! The identity module provides a set of traits and types for generating unique identifiers.
#[doc(inline)]
pub use self::prelude::*;

pub(crate) mod identifier;

mod impls {
    pub mod impl_id;
}

pub mod traits {
    //! this module implements various traits supporting the [`Id`](super::Id) type
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod id;
    pub mod identifier;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::id::*;
        #[doc(inline)]
        pub use super::identifier::*;
    }
}

pub mod types {
    #[doc(inline)]
    pub use self::multi_id::IndexId;

    pub mod multi_id;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::multi_id::IndexId;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::identifier::Id;
    #[doc(inline)]
    pub use super::traits::prelude::*;
    #[doc(inline)]
    pub use super::types::prelude::*;
}

#[cfg(test)]
mod tests {
    use super::Id;
    use super::traits::Identity;

    #[test]
    fn test_id() {
        let id = 0_usize.get();
        assert_eq!(id, &0);
        let atomic = Id::atomic();
        let aid = Id::<usize>::get(&atomic);
        assert_ne!(*aid, *id);
    }
}
