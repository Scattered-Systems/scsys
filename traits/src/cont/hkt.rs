/*
    Appellation: hkt <module>
    Contrib: @FL03
*/

#[allow(dead_code)]
mod old;

/// The [`Hkt`] trait defines an interface for higher-kinded types (HKT).
pub trait Hkt<T> {
    type Cont<A>;
}

/// The [`Functor`] trait extends the [`Hkt`] trait to provide a way to map over its content(s)
/// using a function `f` that transforms values of type `T` into values of type `U`.
pub trait Functor<T>: Hkt<T> {
    fn fmap<F, U>(&self, f: F) -> Self::Cont<U>
    where
        F: Fn(&T) -> U;
}

/*
 *************  Implementations  *************
*/
macro_rules! hkt {
    ($($($t:ident)::*),*) => {
        $(
            hkt!(@impl $($t)::*<T>);
        )*
    };
    (@impl $($t:ident)::*<$T:ident>) => {
        impl<$T> Hkt<$T> for $($t)::*<$T> {
            type Cont<_A> = $($t)::*<_A>;
        }
    };
}

hkt!(core::option::Option);

#[cfg(feature = "alloc")]
hkt! {
    alloc::collections::VecDeque,
    alloc::collections::LinkedList,
    alloc::boxed::Box,
    alloc::rc::Rc,
    alloc::sync::Arc,
    alloc::vec::Vec
}

impl<T> Functor<T> for Option<T> {
    fn fmap<F, U>(&self, f: F) -> Self::Cont<U>
    where
        F: Fn(&T) -> U,
    {
        self.as_ref().map(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option() {
        let opt = Some(42u8);
        let opt2 = opt.fmap(|&x| x as usize + 1);
        assert_eq!(opt2, Some(43_usize));
    }
}
