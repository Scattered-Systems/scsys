/*
    Appellation: applicative <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Applicative
//!
//!

pub trait HKT<U> {
    type C; // Current Type
    type T; // Type C swapped with U
}

pub trait Functor<U>: HKT<U> {
    fn fmap<F>(&self, f: F) -> Self::T
    where
        F: Fn(&Self::C) -> U;
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




/*
    ************* Implementations *************
*/
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

#[allow(unused_macros)]
macro_rules! functor {
    ($($($p:ident)::*),* $(,)?) => {
        $(
            functor!(@impl $($p)::*);
        )*
    };
    (@impl $($p:ident)::*) => {
       impl<T, U> Functor<U> for $($p)::*<T> {
            fn fmap<F>(&self, f: F) -> $($p)::*<U>
            where
                F: Fn(&T) -> U,
            {
                $($p)::*::new(f(self))
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! applicative {
    ($($($p:ident)::*),* $(,)?) => {
        $(
            applicative!(@impl $($p)::*);
        )*
    };
    (@impl $($p:ident)::*) => {
        impl<T, U> Applicative<U> for $($p)::*<T> {
            fn pure_(value: U) -> Self::T {
                $($p)::*::new(value)
            }

            fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> $($p)::*<U>
            where
                F: Fn(&<Self as HKT<U>>::C) -> U,
            {
                let v = fs(self);
                $($p)::*::new(v)
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! monad {
    ($($($p:ident)::*),* $(,)?) => {
        $(
            monad!(@impl $($p)::*);
        )*
    };
    (@impl $($p:ident)::*) => {
        impl<T, U> Monad<U> for $($p)::*<T> {
            fn bind<F>(&self, mut fs: F) -> $($p)::*<U>
            where
                F: FnMut(&T) -> $($p)::*<U>,
            {
                fs(self)
            }
        }
    };
}

hkt!(core::option::Option);

#[cfg(feature = "alloc")]
hkt!(
    alloc::sync::Arc,
    alloc::boxed::Box,
    alloc::rc::Rc,
    alloc::vec::Vec
);

#[cfg(feature = "alloc")]
functor!(alloc::sync::Arc, alloc::boxed::Box, alloc::rc::Rc);

#[cfg(feature = "alloc")]
applicative!(alloc::sync::Arc, alloc::boxed::Box, alloc::rc::Rc);

#[cfg(feature = "alloc")]
monad!(alloc::sync::Arc, alloc::boxed::Box, alloc::rc::Rc);

impl<T, U> Functor<U> for Option<T> {
    fn fmap<F>(&self, f: F) -> Option<U>
    where
        F: Fn(&T) -> U,
    {
        if let &Some(ref value) = self {
            return Some(f(value));
        }
        None
    }
}

#[cfg(feature = "alloc")]
impl<T, U> Functor<U> for Vec<T> {
    fn fmap<F>(&self, f: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        let mut result = Vec::with_capacity(self.len());
        for value in self {
            result.push(f(value));
        }
        result
    }
}

impl<T, U> Applicative<U> for core::option::Option<T> {
    fn pure_(value: U) -> Self::T {
        Some(value)
    }

    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> Option<U>
    where
        F: Fn(&T) -> U,
    {
        match *self {
            Some(ref value) => match fs {
                Some(f) => Some(f(value)),
                None => None,
            },
            None => None,
        }
    }
}

#[cfg(feature = "alloc")]
impl<T, U> Applicative<U> for alloc::vec::Vec<T> {
    fn pure_(value: U) -> Self::T {
        vec![value]
    }

    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> alloc::vec::Vec<U>
    where
        F: Fn(&T) -> U,
    {
        let mut result = alloc::vec::Vec::new();
        for (i, f) in fs.into_iter().enumerate() {
            let v = (f)(&self[i]);
            result.push(v)
        }
        return result;
    }
}

impl<T, U> Monad<U> for core::option::Option<T> {
    fn bind<F>(&self, mut fs: F) -> core::option::Option<U>
    where
        F: FnMut(&T) -> core::option::Option<U>,
    {
        match self {
            Some(x) => fs(x),
            None => None,
        }
    }
}

#[cfg(feature = "alloc")]
impl<T, U> Monad<U> for alloc::vec::Vec<T> {
    fn bind<F>(&self, mut fs: F) -> Vec<U>
    where
        F: FnMut(&T) -> alloc::vec::Vec<U>,
    {
        let mut v = alloc::vec::Vec::new();
        for x in self {
            v.extend(fs(x));
        }
        v
    }
}

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

