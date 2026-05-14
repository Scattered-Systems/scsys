/*
   Appellation: incremental <traits>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use num_traits::One;

/// [`Decrement`] defines an interface for decrementing a value.
pub trait Decrement {
    type Output;

    fn dec(self) -> Self::Output;
}
/// [`DecrementRef`] is a trait describing the ability to decrement a value by reference.
pub trait DecrementRef {
    type Output;

    fn dec_ref(&self) -> Self::Output;
}
/// [`DecrementMut`] is a trait describing the ability to decrement a value in place. It is
/// similar to [`Decrement`], but it allows for mutable access to the value.
///
/// **note**: the automatic implementation of `DecrementMut` leverages the [`replace`](core::mem::replace)
/// method to compute the next value, replacing and returning the previous value.
pub trait DecrementMut {
    type Output;

    fn dec_mut(&mut self) -> Self::Output;
}
/// [`Increment`] defines an interface for incrementing a value.
pub trait Increment {
    type Output;

    fn inc(self) -> Self::Output;
}
/// [`IncrementRef`] is similar to [`Increment`], but it allows for incrementing a value by
/// reference.
pub trait IncrementRef {
    type Output;

    fn inc_ref(&self) -> Self::Output;
}
/// [`IncrementMut`] is similar to [`Increment`], but it allows for incrementing a value in place.
///
/// **note**: the automatic implementation of `IncrementMut` leverages the [`replace`](core::mem::replace)
/// method to compute the next value, replacing and returning the previous value.
pub trait IncrementMut {
    type Output;

    fn inc_mut(&mut self) -> Self::Output;
}

/*
 ******** implementations ********
*/
impl<S> Decrement for S
where
    S: One + core::ops::Sub<S, Output = S>,
{
    type Output = S;

    fn dec(self) -> Self::Output {
        self - S::one()
    }
}

impl<S> DecrementMut for S
where
    S: One,
    for<'a> &'a S: core::ops::Sub<S, Output = S>,
{
    type Output = S;

    fn dec_mut(&mut self) -> Self::Output {
        let next = &(*self) - S::one();
        core::mem::replace(self, next)
    }
}

impl<S> DecrementRef for S
where
    S: One,
    for<'a> &'a S: core::ops::Sub<S, Output = S>,
{
    type Output = S;

    fn dec_ref(&self) -> Self::Output {
        self - S::one()
    }
}

impl<S> Increment for S
where
    S: One + core::ops::Add<S, Output = S>,
{
    type Output = S;

    fn inc(self) -> Self::Output {
        self + S::one()
    }
}

impl<S> IncrementMut for S
where
    S: One,
    for<'a> &'a S: core::ops::Add<S, Output = S>,
{
    type Output = S;

    fn inc_mut(&mut self) -> Self::Output {
        let next = &(*self) + S::one();
        core::mem::replace(self, next)
    }
}

impl<S> IncrementRef for S
where
    S: One,
    for<'a> &'a S: core::ops::Add<S, Output = S>,
{
    type Output = S;

    fn inc_ref(&self) -> Self::Output {
        self + S::one()
    }
}
