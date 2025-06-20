/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This module focuses on the [`Id`] implementation, a generic wrapper type used to define 
//! various identifiers. The `Id` type is equipped with additional methods and traits to 
//! facilitate the creation and management of identifiers in a type-safe manner.
#[doc(inline)]
pub use self::{identifier::Id, traits::*, types::*};

mod identifier;

mod impls {
    pub mod impl_id;
    pub mod impl_id_repr;
}

mod traits {
    //! this module implements various traits supporting the [`Id`](super::Id) type
    #[doc(inline)]
    pub use self::prelude::*;

    mod id;
    mod identifier;

    mod prelude {
        #[doc(inline)]
        pub use super::id::*;
        #[doc(inline)]
        pub use super::identifier::*;
    }
}

mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    mod multi_id;

    mod prelude {
        #[doc(inline)]
        pub use super::multi_id::IndexId;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::identifier::Id;
    #[doc(inline)]
    pub use super::traits::*;
    #[doc(inline)]
    pub use super::types::*;
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
