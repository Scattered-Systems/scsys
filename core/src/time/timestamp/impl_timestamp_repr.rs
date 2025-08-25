/*
    appellation: impl_timestamp_repr <module>
    authors: @FL03
*/
use crate::time::{RawTimestamp, Timestamp};

impl<T> Timestamp<&T>
where
    T: RawTimestamp,
{
    /// returns a new [`Timestamp`] containing a clone of the current value
    pub fn cloned(&self) -> Timestamp<T>
    where
        T: Clone,
    {
        Timestamp(self.0.clone())
    }
    /// returns a new [`Timestamp`] containing a copy of the current value
    pub fn copied(&self) -> Timestamp<T>
    where
        T: Copy,
    {
        Timestamp(*self.0)
    }
}

impl<T> Timestamp<&mut T>
where
    T: RawTimestamp,
{
    /// returns a new [`Timestamp`] containing a clone of the current value
    pub fn cloned(&self) -> Timestamp<T>
    where
        T: Clone,
    {
        Timestamp(self.0.clone())
    }
    /// returns a new [`Timestamp`] containing a copy of the current value
    pub fn copied(&self) -> Timestamp<T>
    where
        T: Copy,
    {
        Timestamp(*self.0)
    }
}
