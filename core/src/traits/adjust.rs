/*
   Appellation: adjust <traits>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::ops::{Add, Sub};
use num::traits::One;

pub trait Adjust<T> {
    type Output;

    fn adjust(&mut self, adjust: T) -> Self::Output;
}

/// `Decrement` is a trait describing the ability to decrement a value.
/// 
pub trait Decrement {
    type Output;

    fn dec(&self) -> Self::Output;
}

pub trait Increment {
    type Output;

    fn inc(&self) -> Self::Output;
}

/*
 ******** implementations ********
*/
impl<S, T> Decrement for S where S: One, for<'a> &'a S: Sub<S, Output = T> {
    type Output = T;

    fn dec(&self) -> Self::Output {
        self - S::one()
    }
}

impl<S, T> Increment for S where S: One, for<'a> &'a S: Add<S, Output = T> {
    type Output = T;

    fn inc(&self) -> Self::Output {
        self + S::one()
    }
}