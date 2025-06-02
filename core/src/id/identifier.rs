/*
    Appellation: atomic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// [`Id`] is a generic identifier type that wraps a value of type `T`.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct Id<T = usize>(pub T);

impl<T> Id<T> {
    /// Create a new identifier with the default value
    pub fn new() -> Self
    where
        T: Default,
    {
        Self(T::default())
    }
    /// create a new identifier from the given value
    pub fn from_value(id: T) -> Self {
        Self(id)
    }
    #[cfg(feature = "rand")]
    pub fn random() -> Self
    where
        rand_distr::StandardUniform: rand_distr::Distribution<T>,
    {
        use rand::Rng;
        let mut rng = rand::rng();
        Self::from_value(rng.random())
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
        Id(id)
    }
    /// apply a function onto the inner value and return a new instance with the result
    pub fn map<U, F>(self, f: F) -> Id<U>
    where
        F: FnOnce(T) -> U,
    {
        Id(f(self.value()))
    }
}

impl Id<usize> {
    pub fn atomic() -> Self {
        use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self(COUNTER.fetch_add(1, Relaxed))
    }

    pub fn next(&self) -> Self {
        Self::atomic()
    }
}

#[cfg(feature = "uuid")]
impl Id<uuid::Uuid> {
    pub fn v3(namespace: &uuid::Uuid, name: &[u8]) -> Self {
        let id = uuid::Uuid::new_v3(&namespace, name);
        Self(id)
    }

    #[cfg(all(feature = "rng", feature = "uuid"))]
    pub fn v4() -> Self {
        let id = uuid::Uuid::new_v4();
        Self(id)
    }
}

impl<T> crate::id::Identifier for Id<T> {
    seal!();
}

impl<T: Default> Default for Id<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AsRef<T> for Id<T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T> AsMut<T> for Id<T> {
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::borrow::Borrow<T> for Id<T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T> core::borrow::BorrowMut<T> for Id<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::ops::Deref for Id<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> core::ops::DerefMut for Id<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<Q> PartialEq<Q> for Id<Q>
where
    Q: PartialEq,
{
    fn eq(&self, other: &Q) -> bool {
        self.get() == other
    }
}

impl<Q> PartialOrd<Q> for Id<Q>
where
    Q: PartialOrd,
{
    fn partial_cmp(&self, other: &Q) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

crate::fmt_wrapper! {
    Id<T>(
        Binary,
        Debug,
        Display,
        LowerExp,
        LowerHex,
        Octal,
        Pointer,
        UpperExp,
        UpperHex
    )
}
