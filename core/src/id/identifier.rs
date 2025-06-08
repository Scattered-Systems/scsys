/*
    Appellation: id <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Identifier;

/// [`Id`] is a generic identifier type that wraps a value of type `T`.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct Id<T = usize>(pub T);

impl<T> Id<T> {
    /// create a new identifier from the given value
    pub const fn new(id: T) -> Self {
        Self(id)
    }
    /// Create a new identifier with the default value
    pub fn default() -> Self
    where
        T: Default,
    {
        Self::new(T::default())
    }
    /// returns a new id with a value of `1`
    pub fn one() -> Self
    where
        T: num_traits::One,
    {
        Self::new(T::one())
    }
    /// returns a new id with a value of `0`
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Self::new(T::zero())
    }
    #[cfg(feature = "rand")]
    pub fn random() -> Self
    where
        rand_distr::StandardUniform: rand_distr::Distribution<T>,
    {
        use rand::Rng;
        let mut rng = rand::rng();
        Self::new(rng.random())
    }
    /// returns an immutable reference to the inner value
    pub const fn get(&self) -> &T {
        &self.0
    }
    /// returns a mutable reference to the inner value
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    /// consumes the current instance to return the inner value
    #[inline]
    pub fn value(self) -> T {
        self.0
    }
    /// use the [`replace`](core::mem::replace) method to update and return the inner value
    pub const fn replace(&mut self, id: T) -> T {
        core::mem::replace(self.get_mut(), id)
    }
    /// mutate the inner value and return a mutable reference to the wrapper for chaining
    pub fn set(&mut self, id: T) -> &mut Self {
        *self.get_mut() = id;
        self
    }
    /// [`swap`](core::mem::swap) the inner value with that of another identifier instance of
    /// the same type `T`
    pub const fn swap(&mut self, id: &mut Id<T>) {
        core::mem::swap(self.get_mut(), id.get_mut())
    }
    /// [`take`](core::mem::take) the inner value, leaving the logical [`default`](Default::default)
    /// value in its place.
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.get_mut())
    }
    /// consumes the current instance to replace it with another.
    pub fn with<U>(self, id: U) -> Id<U> {
        Id::new(id)
    }
    /// apply a function onto the inner value and return a new instance with the result
    pub fn map<U, F>(self, f: F) -> Id<U>
    where
        F: FnOnce(T) -> U,
    {
        Id::new(f(self.value()))
    }
    /// replaces the current id with the next logical value of type `T`
    ///
    /// ```rust
    /// use scsys::Id;
    ///
    /// let mut id = Id::zero();
    /// assert_eq!(id.step(), 0);
    /// assert_eq!(id.step(), 1);
    ///
    /// ```
    pub fn step(&mut self) -> T
    where
        T: num_traits::One,
        for<'a> &'a T: core::ops::Add<T, Output = T>,
    {
        self.replace(self.get() + T::one())
    }
    /// returns a new identifier with a reference to the inner value
    pub const fn view(&self) -> Id<&T> {
        Id::new(self.get())
    }
    /// returns a new identifier with a mutable reference to the inner value
    pub const fn view_mut(&mut self) -> Id<&mut T> {
        Id::new(self.get_mut())
    }
}

impl Id<usize> {
    pub fn atomic() -> Self {
        use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self::new(COUNTER.fetch_add(1, Relaxed))
    }
    /// replaces the current id with the atomic-ally next value and returns the previous value.
    /// see [`step`](Id::step) for more information
    pub fn atomic_step(&mut self) -> usize {
        use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        self.replace(COUNTER.fetch_add(1, Relaxed))
    }
}

#[cfg(feature = "uuid")]
impl Id<uuid::Uuid> {
    pub fn v3(namespace: &uuid::Uuid, name: &[u8]) -> Self {
        let id = uuid::Uuid::new_v3(namespace, name);
        Self::new(id)
    }

    #[cfg(all(feature = "rng", feature = "uuid"))]
    pub fn v4() -> Self {
        let id = uuid::Uuid::new_v4();
        Self::new(id)
    }
}

impl<T> Default for Id<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Identifier for Id<T> {
    seal!();
}
