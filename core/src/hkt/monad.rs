/*
    Appellation: monad <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Applicative;
use super::HKT;

use super::containers::*;

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

#[allow(unused_macros)]
macro_rules! monad {
    ($($t:ident),* $(,)?) => {
        $(
            monad!(@impl $t);
        )*
    };
    (@impl $t:ident) => {
        impl<T, U> Monad<U> for $t<T> {
            fn bind<F>(&self, mut fs: F) -> $t<U>
            where
                F: FnMut(&T) -> $t<U>,
            {
                fs(self)
            }
        }
    };
}

#[cfg(feature = "alloc")]
monad!(Arc, Box, Rc);

impl<T, U> Monad<U> for Option<T> {
    fn bind<F>(&self, mut fs: F) -> Option<U>
    where
        F: FnMut(&T) -> Option<U>,
    {
        match self {
            Some(x) => fs(x),
            None => None,
        }
    }
}

#[cfg(feature = "alloc")]
impl<T, U> Monad<U> for Vec<T> {
    fn bind<F>(&self, mut fs: F) -> Vec<U>
    where
        F: FnMut(&T) -> Vec<U>,
    {
        let mut v = Vec::new();
        for x in self {
            v.extend(fs(x));
        }
        v
    }
}
