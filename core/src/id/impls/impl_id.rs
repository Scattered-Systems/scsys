/*
    appellation: impl_id <module>
    authors: @FL03
*/
use crate::id::Id;

impl<T> Id<&T> {
    /// returns a new identifier with a cloned inner value
    pub fn cloned(&self) -> Id<T>
    where
        T: Clone,
    {
        Id(self.0.clone())
    }
    /// returns a new identifier with the inner value copied
    pub fn copied(&self) -> Id<T>
    where
        T: Copy,
    {
        Id(*self.0)
    }
}

impl<T> Id<&mut T> {
    /// returns a new identifier with a cloned inner value
    pub fn cloned(&self) -> Id<T>
    where
        T: Clone,
    {
        Id(self.0.clone())
    }
    /// returns a new identifier with the inner value copied
    pub fn copied(&self) -> Id<T>
    where
        T: Copy,
    {
        Id(*self.0)
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

impl<'a, Q> PartialEq<&'a Q> for Id<Q>
where
    Q: PartialEq,
{
    fn eq(&self, other: &&'a Q) -> bool {
        self.get() == *other
    }
}

impl<'a, Q> PartialEq<&'a mut Q> for Id<Q>
where
    Q: PartialEq,
{
    fn eq(&self, other: &&'a mut Q) -> bool {
        self.get() == *other
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
