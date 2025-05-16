/*
    Appellation: raw_error <module>
    Contrib: @FL03
*/
/// A type alias for a [`Err`] whose generic type is a [`Box`] of a trait object
/// implementing [`core::error::Error`].
pub type BoxErr = RawError<Box<dyn core::error::Error + Send + Sync + 'static>>;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent, rename_all = "snake_case")
)]
#[repr(transparent)]
pub struct RawError<U: core::error::Error> {
    inner: U,
}

impl<U> RawError<U>
where
    U: core::error::Error,
{
    /// returns a new instance wrapping the given error.
    pub fn from_err(inner: U) -> Self {
        Self { inner }
    }
    /// consumes the wrapper to return the inner value.
    pub fn into_inner(self) -> U {
        self.inner
    }
    /// returns an immutable reference to the underlying error.
    pub const fn get(&self) -> &U {
        &self.inner
    }
    /// returns a mutable reference to the underlying error.
    #[inline]
    pub fn get_mut(&mut self) -> &mut U {
        &mut self.inner
    }
    /// uses the [`replace`](core::mem::replace) method to replace and return the current error
    /// with another.
    #[inline]
    pub fn replace(&mut self, new: U) -> U {
        core::mem::replace(&mut self.inner, new)
    }
    /// update the inner value before returning a mutable reference to the wrapper;
    #[inline]
    pub fn set(&mut self, new: U) -> &mut Self {
        self.inner = new;
        self
    }
    /// swap out the inner values of two instances of [`RawError`].
    pub fn swap(&mut self, other: &mut Self) {
        core::mem::swap(&mut self.inner, &mut other.inner)
    }
    /// apply a function to the error and return a new instance with the result.
    pub fn map<V, F>(self, f: F) -> RawError<V>
    where
        F: FnOnce(U) -> V,
        V: core::error::Error,
    {
        RawError::from_err(f(self.inner))
    }
    /// mutate the error using the given function
    pub fn map_mut<F>(&mut self, f: F)
    where
        F: FnOnce(&mut U),
    {
        f(&mut self.inner)
    }
}

impl<E> From<E> for RawError<E>
where
    E: core::error::Error,
{
    fn from(inner: E) -> Self {
        Self::from_err(inner)
    }
}

impl<E: core::error::Error> core::error::Error for RawError<E> {}

impl<E> core::fmt::Display for RawError<E>
where
    E: core::error::Error,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
