/*
    Appellation: wrapper <module>
    Contrib: @FL03
*/
/// [IntoInner] is typically used for basic structures that wrap a single value.
pub trait IntoInner {
    type Inner;

    fn into_inner(self) -> Self::Inner;
}
/// A [`RawWrapper`] is the base trait defining all so-called "wrapper" types.
pub trait RawWrapper {
    type Item;
}
/// The [`Wrapper`] trait is used to establish a common interface for all simplemented
/// structures that "wrap" a single value. Essentially, any type capable of implementing
/// `#[repr(transparent)]` can be considered a wrapper.
pub trait Wrapper: RawWrapper {
    /// returns a new instance of the wrapper initialized with the default value
    fn new() -> Self
    where
        Self: Sized,
        Self::Item: Default,
    {
        Self::from_value(Self::Item::default())
    }
    /// returns a new instance of the wrapper initialized with the given value
    fn from_value(value: Self::Item) -> Self;
    /// consumes the wrapper to return the stored value
    fn into_inner(self) -> Self::Item;
    /// returns an immutable reference to the stored value
    fn get(&self) -> &Self::Item;
    /// returns a mutable reference to the stored value
    fn get_mut(&mut self) -> &mut Self::Item;
}
/// [`WrapperExt`] is an automatically implemented trait extending the [`Wrapper`] trait with
/// additional
pub trait WrapperExt: Wrapper {
    /// [`replace`](core::mem::replace) replaces the value inside the wrapper with a new one,
    /// returning the old value.
    fn replace(&mut self, value: Self::Item) -> Self::Item {
        core::mem::replace(self.get_mut(), value)
    }
    /// update the current value before returning a mutable reference to the wrapper
    fn set(&mut self, value: Self::Item) -> &mut Self {
        *self.get_mut() = value;
        self
    }
    /// [`swap`](core::mem::swap) swaps the inner values of two instances.
    fn swap(&mut self, other: &mut Self) {
        core::mem::swap(self.get_mut(), other.get_mut())
    }

    fn take(&mut self) -> Self::Item
    where
        Self::Item: Default,
    {
        core::mem::take(self.get_mut())
    }
}

pub trait Mapper<U> {
    type Item<V>;

    fn map<V, F>(self, f: F) -> Self::Item<V>
    where
        F: FnOnce(U) -> V;
}

/*
 ************* Implementations *************
*/
impl<T, U> WrapperExt for T where T: Wrapper<Item = U> {}
