/*
    Appellation: hkt <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Higher Kinded Types
//!
//!
#[doc(inline)]
pub use self::{applicative::*, functor::*, monad::*};

mod applicative;
mod functor;
mod monad;

pub(crate) mod prelude {
    pub use super::applicative::Applicative;
    pub use super::functor::Functor;
    pub use super::monad::Monad;
    pub use super::HKT;
}

pub(crate) mod containers {
    pub(crate) use core::option::Option;

    #[cfg(feature = "alloc")]
    pub(crate) use alloc::{boxed::Box, rc::Rc, sync::Arc, vec::Vec};
}

pub trait HKT<U> {
    type C; // Current Type
    type T; // Type C swapped with U
}

#[macro_export]
macro_rules! hkt {
    ($($($p:ident)::*),* $(,)?) => {
        $(
            $crate::hkt!(@impl $($p)::*);
        )*
    };
    (@impl $($p:ident)::*) => {
        impl<T, U> HKT<U> for $($p)::*<T> {
            type C = T;
            type T = $($p)::*<U>;
        }
    };
}

hkt!(containers::Option);

#[cfg(feature = "alloc")]
hkt!(
    containers::Arc,
    containers::Box,
    containers::Rc,
    containers::Vec
);

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_hkt_vec() {
        let v = Vec::from_iter(0..9);
        let v2 = v.fmap(|x| (x + 1).to_string());
        assert_eq!(v2, ["1", "2", "3", "4", "5", "6", "7", "8", "9"]);

        let v = Vec::return_(0);
        let v2 = v.bind(|x| vec![x + 1]);
        assert_eq!(v2, [1]);
    }
}
