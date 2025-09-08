/*
    appellation: impl_deprecated <module>
    authors: @FL03
*/
use super::Timestamp;
use crate::traits::RawTimestamp;

#[doc(hidden)]
impl<T> Timestamp<T>
where
    T: RawTimestamp,
{
    #[deprecated(
        since = "0.2.8",
        note = "use `get` instead; this will be removed in the next major release"
    )]
    pub fn as_ref(&self) -> &T {
        self.get()
    }
    #[deprecated(
        since = "0.2.8",
        note = "use `get_mut` instead; this will be removed in the next major release"
    )]
    pub fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
    #[deprecated(
        since = "0.3.1",
        note = "use `into_innter` instead; this will be removed in the next major release"
    )]
    pub fn value(self) -> T {
        self.0
    }
}
