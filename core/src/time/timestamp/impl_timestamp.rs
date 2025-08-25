/*
    appellation: impl_timestamp <module>
    authors: @FL03
*/
use crate::time::{RawTimestamp, Timestamp};

impl<T: RawTimestamp> AsRef<T> for Timestamp<T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T: RawTimestamp> AsMut<T> for Timestamp<T> {
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T: RawTimestamp> core::borrow::Borrow<T> for Timestamp<T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T: RawTimestamp> core::borrow::BorrowMut<T> for Timestamp<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T: RawTimestamp> core::ops::Deref for Timestamp<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T: RawTimestamp> core::ops::DerefMut for Timestamp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

crate::fmt_wrapper! {
    Timestamp<T>(
        Binary,
        Octal,
        LowerHex,
        UpperHex,
        Display,
        Debug,
        LowerExp,
        UpperExp,
        Pointer,
    )
}

impl From<core::time::Duration> for Timestamp<u64> {
    fn from(dur: core::time::Duration) -> Self {
        Self(dur.as_secs())
    }
}

impl From<core::time::Duration> for Timestamp<u128> {
    fn from(dur: core::time::Duration) -> Self {
        Self(dur.as_millis())
    }
}

impl From<Timestamp<u64>> for core::time::Duration {
    fn from(ts: Timestamp<u64>) -> Self {
        Self::from_secs(*ts)
    }
}

impl From<Timestamp<u128>> for core::time::Duration {
    fn from(ts: Timestamp<u128>) -> Self {
        Self::from_millis(*ts as u64)
    }
}

#[cfg(feature = "chrono")]
impl<Tz> From<chrono::DateTime<Tz>> for Timestamp<i64>
where
    Tz: chrono::TimeZone,
{
    fn from(ts: chrono::DateTime<Tz>) -> Self {
        Self(ts.timestamp())
    }
}
