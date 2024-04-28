/*
    Appellation: applicative <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Applicative
//!
//!
use super::functor::Functor;
use super::HKT;

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, rc::Rc, sync::Arc, vec::Vec};

#[cfg(feature = "std")]
use std::{rc::Rc, sync::Arc};

pub trait Applicative<U>: Functor<U> {
    fn pure_(value: U) -> Self::T
    where
        Self: HKT<U, C = U>;
    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<U>>::T
    where
        F: Fn(&<Self as HKT<U>>::C) -> U,
        Self: HKT<F>;
}

macro_rules! applicative {
    ($($t:ident($e:expr)),* $(,)?) => {
        $(
            applicative!(@impl $t($e));
        )*
    };
    (@impl $t:ident($e:expr)) => {
        impl<T, U> Applicative<U> for $t<T> {
            fn pure_(value: U) -> Self::T {
                $e(value)
            }

            fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> $t<U>
            where
                F: Fn(&<Self as HKT<U>>::C) -> U,
            {
                let v = fs(self);
                $e(v)
            }
        }
    };
}

applicative!(Arc(Arc::new), Box(Box::new), Rc(Rc::new));



impl<T, U> Applicative<U> for core::option::Option<T> {
    fn pure_(value: U) -> Self::T {
        Some(value)
    }

    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> core::option::Option<U>
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

impl<T, U> Applicative<U> for Vec<T> {
    fn pure_(value: U) -> Self::T {
        vec![value]
    }

    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        let mut result = Vec::new();
        for (i, f) in fs.into_iter().enumerate() {
            let v = (f)(&self[i]);
            result.push(v)
        }
        return result;
    }
}
