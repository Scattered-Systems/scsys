/*
    Appellation: raw_error <module>
    Contrib: @FL03
*/

#[cfg(feature = "alloc")]
/// A type alias for a [`Err`] whose generic type is a [`Box`] of a trait object
/// implementing [`core::error::Error`].
pub type BoxErr = ErrorBase<alloc::boxed::Box<dyn core::error::Error + Send + Sync + 'static>>;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default, transparent, rename_all = "snake_case")
)]
#[repr(transparent)]
pub struct ErrorBase<E>
where
    E: core::error::Error,
{
    inner: E,
}

impl<E> ErrorBase<E>
where
    E: core::error::Error,
{
    /// returns a new instance wrapping the given error.
    pub fn from_err(inner: E) -> Self {
        Self { inner }
    }
    /// consumes the wrapper to return the inner value.
    pub fn into_inner(self) -> E {
        self.inner
    }
    /// returns an immutable reference to the underlying error.
    pub const fn get(&self) -> &E {
        &self.inner
    }
    /// returns a mutable reference to the underlying error.
    #[inline]
    pub fn get_mut(&mut self) -> &mut E {
        &mut self.inner
    }
    /// uses the [`replace`](core::mem::replace) method to replace and return the current error
    /// with another.
    #[inline]
    pub fn replace(&mut self, new: E) -> E {
        core::mem::replace(&mut self.inner, new)
    }
    /// update the inner value before returning a mutable reference to the wrapper;
    #[inline]
    pub fn set(&mut self, new: E) -> &mut Self {
        self.inner = new;
        self
    }
    /// swap out the inner values of two instances of [`RawError`].
    pub fn swap(&mut self, other: &mut Self) {
        core::mem::swap(&mut self.inner, &mut other.inner)
    }
    /// apply a function to the error and return a new instance with the result.
    pub fn map<V, F>(self, f: F) -> ErrorBase<V>
    where
        F: FnOnce(E) -> V,
        V: core::error::Error,
    {
        ErrorBase::from_err(f(self.inner))
    }
    /// mutate the error using the given function
    pub fn map_mut<F>(&mut self, f: F)
    where
        F: FnOnce(&mut E),
    {
        f(&mut self.inner)
    }
}

impl<E> From<E> for ErrorBase<E>
where
    E: core::error::Error,
{
    fn from(inner: E) -> Self {
        Self::from_err(inner)
    }
}

impl<'a, E> From<&'a ErrorBase<E>> for ErrorBase<E>
where
    E: core::error::Error + Clone,
{
    fn from(inner: &'a ErrorBase<E>) -> Self {
        Self::from_err(inner.inner.clone())
    }
}

impl<E: core::error::Error> core::error::Error for ErrorBase<E> {}

impl<E> core::fmt::Display for ErrorBase<E>
where
    E: core::error::Error,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
