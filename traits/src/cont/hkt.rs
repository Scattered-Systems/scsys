/*
    Appellation: hkt <module>
    Contrib: @FL03
*/

pub trait Hkt {
    type C<A>;
}

pub trait Functor<T>: Hkt {
    fn fmap<F, U>(&self, f: F) -> Self::C<U>
    where
        F: Fn(&T) -> U;
}
#[doc(hidden)]
#[allow(unused)]
mod old {

    pub trait HKT<U> {
        type C; // Current Type
        type T; // Type C swapped with U
    }

    pub trait Functor<U>: HKT<U> {
        fn fmap<F>(self, f: F) -> Self::T
        where
            F: Fn(Self::C) -> U;
    }

    pub trait Applicative<U>: Functor<U> {
        fn pure_(value: U) -> Self::T
        where
            Self: HKT<U, C = U>;
        fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<U>>::T
        where
            F: Fn(&<Self as HKT<U>>::C) -> U,
            Self: HKT<F>;
    }

    pub trait Monad<U>: Applicative<U> {
        fn return_(x: U) -> Self::T
        where
            Self: HKT<U, C = U>,
        {
            Self::pure_(x)
        }

        fn bind<F>(&self, fs: F) -> Self::T
        where
            F: FnMut(&Self::C) -> Self::T;

        fn join<T>(&self) -> T
        where
            Self: HKT<U, T = T, C = T>,
            T: Clone,
        {
            self.bind(|x| x.clone())
        }
    }
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
        impl<T> Hkt for $($t)::*<T> {
            type C<A> = $($t)::*<A>;
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
    fn fmap<F, U>(&self, f: F) -> Self::C<U>
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
