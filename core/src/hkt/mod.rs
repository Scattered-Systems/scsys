/*
    Appellation: hkt <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Higher Kinded Types
//!
//!
pub use self::prelude::*;

pub mod applicative;
pub mod functor;
pub mod monad;

pub(crate) mod containers {
    pub(crate) use core::option::Option;

    #[cfg(all(feature = "alloc", no_std))]
    pub(crate) use alloc::{boxed::Box, rc::Rc, sync::Arc, vec::Vec};

    #[cfg(feature = "std")]
    pub(crate) use std::{boxed::Box, rc::Rc, sync::Arc, vec::Vec};
}

pub trait HKT<U> {
    type C; // Current Type
    type T; // Type C swapped with U
}

#[macro_export]
macro_rules! hkt {
    ($($($p:ident)::*),*) => {
        $(
            hkt!(@impl $($p)::*);
        )*
    };
    (@impl $($p:ident)::*) => {
        impl<T, U> HKT<U> for $($p)::*<T> {
            type C = T;
            type T = $($p)::*<U>;
        }
    };
}

hkt!(
    containers::Arc,
    containers::Box,
    containers::Option,
    containers::Rc,
    containers::Vec
);

pub(crate) mod prelude {
    pub use super::applicative::Applicative;
    pub use super::functor::Functor;
    pub use super::monad::Monad;
    pub use super::HKT;
}

#[cfg(test)]
mod tests {

    use super::functor::Functor;
    use super::monad::Monad;

    #[test]
    fn test_hkt_vec() {
        let v = Vec::from_iter(0..9);
        let v2 = v.fmap(|x| (x + 1).to_string());
        assert_eq!(v2, vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]);

        let v = Vec::return_(0);
        let v2 = v.bind(|x| vec![x + 1]);
        assert_eq!(v2, vec![1]);
    }
}
