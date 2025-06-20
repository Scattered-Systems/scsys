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
