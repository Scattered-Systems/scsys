/*
    Appellation: functor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Functor
//!
//! A functor is a type that when mapped over, preserves the structure of the type while applying a function to the values within the type.
//! Functors are useful for modeling the functional effects on values of parameterized data types.
use super::HKT;
#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, rc::Rc, sync::Arc, vec::Vec};

#[cfg(feature = "std")]
use std::{rc::Rc, sync::Arc};

pub trait Functor<U>: HKT<U> {
    fn fmap<F>(&self, f: F) -> Self::T
    where
        F: Fn(&Self::C) -> U;
}

macro_rules! functor {
    ($($t:ident),* $(,)?) => {
        $(
            functor!(@impl $t);
        )*
    };
    (@impl $t:ident) => {
       impl<T, U> Functor<U> for $t<T> {
            fn fmap<F>(&self, f: F) -> $t<U>
            where
                F: Fn(&T) -> U,
            {
                $t::new(f(self))
            }
        } 
    };
}

functor!(Arc, Box, Rc);

impl<T, U> Functor<U> for Option<T> {
    fn fmap<F>(&self, f: F) -> Option<U>
    where
        F: Fn(&T) -> U,
    {
        if let Some(ref value) = self {
            return Some(f(value));
        }
        None
    }
}


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
