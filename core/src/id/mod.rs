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

    #[test]
    fn test_id() {
        let mut id = Id::<usize>::default();
        assert_eq!(id.get(), &0);
        assert_eq!(id.get_mut(), &mut 0);
        let view = id.view();
        assert_eq!(view.get(), &&0);
        assert_eq!(view.copied(), id);
    }

    #[test]
    fn test_atomic_id() {
        let v1 = Id::atomic();
        let v2 = Id::atomic();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_id_stepper() {
        let mut id: Id<usize> = Id::zero();
        assert_eq!(id.step(), 0);
        assert_eq!(id.step(), 1);
        assert_eq!(id.step(), 2);
        assert_eq!(id.step(), 3);
        assert_eq!(id.step(), 4);
        assert_eq!(id, 5);
    }
}
