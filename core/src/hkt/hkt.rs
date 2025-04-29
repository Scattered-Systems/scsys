/*
    Appellation: hkt <module>
    Contrib: @FL03
*/

pub trait HKT {
    type C<A>;
}

pub trait Functor<T>: HKT {
    fn fmap<F, Z>(&self, f: F) -> Self::C<Z>
    where
        F: Fn(&Self::C<T>) -> Self::C<Z>;
}

/*
 *************  Implementations  *************
*/
macro_rules! hkt {
    ($($($t:ident)::*),*) => {
        $(
            hkt!(@impl $($t)::*);
        )*
    };
    (@impl $($t:ident)::*) => {
        impl<T> HKT for $($t)::*<T> {
            type C<A> = $($t)::*<A>;
        }
    };
}

macro_rules! functor {
    ($($($t:ident)::*),*) => {
        $(
            functor!(@impl $($t)::*);
        )*
    };
    (@impl $($t:ident)::*) => {
        impl<T> Functor<T> for $($t)::*<T> {
            fn fmap<F, Z>(&self, f: F) -> Self::C<Z>
            where
                F: Fn(&Self::C<T>) -> Self::C<Z> {
                f(self)
            }
        }
    };
}

hkt!(core::option::Option);
functor!(core::option::Option);

#[cfg(feature = "alloc")]
hkt! {
    alloc::collections::VecDeque,
    alloc::collections::LinkedList,
    alloc::boxed::Box,
    alloc::rc::Rc,
    alloc::sync::Arc,
    alloc::vec::Vec
}

#[cfg(feature = "alloc")]
functor! {
    alloc::collections::VecDeque,
    alloc::collections::LinkedList,
    alloc::boxed::Box,
    alloc::rc::Rc,
    alloc::sync::Arc,
    alloc::vec::Vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::FromPrimitive;

    #[test]
    fn test_option() {
        let opt = Some(42u8);
        let opt2 = opt.fmap(|x| x.map(|i| usize::from_u8(i).unwrap() + 1));
        assert_eq!(opt2, Some(43usize));
    }
}
