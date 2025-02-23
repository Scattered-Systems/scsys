/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::time::Now;
use core::time::Duration;

/// [Timestamp] is a generic type used to represent a timestamp.
///
/// The timestamp considers the standard timestamp to be the
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Timestamp<T = u64>(pub T);

impl<T> Timestamp<T> {
    pub fn new(ts: T) -> Self {
        Self(ts)
    }
    /// Get the current timestamp.
    pub fn now() -> Self
    where
        Self: Now<Output = Self>,
    {
        <Self as Now>::now()
    }
    /// Get an immutable reference to the current timestamp.
    pub fn as_ref(&self) -> &T {
        &self.0
    }
    /// Get a mutable reference to the current timestamp.
    pub fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
    /// Get the current timestamp.
    pub fn get(self) -> T {
        self.0
    }
    /// Replace the current timestamp with a new one.
    pub fn replace(&mut self, ts: T) -> T {
        core::mem::replace(&mut self.0, ts)
    }
    /// Set the current timestamp to a new value.
    pub fn set(&mut self, ts: T) {
        self.0 = ts;
    }
    /// Take the current timestamp and replace it with the default value.
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(&mut self.0)
    }

    pub fn update(&mut self, ts: T) {
        self.0 = ts;
    }
}

impl<T> Default for Timestamp<T>
where
    Self: Now<Output = Self>,
{
    fn default() -> Self {
        Self::now()
    }
}

#[cfg(feature = "std")]
impl Now for Timestamp<u64> {
    type Output = Self;

    fn now() -> Self::Output {
        Self::new(super::systime().as_secs())
    }
}

#[cfg(feature = "std")]
impl Now for Timestamp<u128> {
    type Output = Self;

    fn now() -> Self::Output {
        Self::new(super::systime().as_millis())
    }
}

#[cfg(feature = "chrono")]
impl Now for Timestamp<i64> {
    type Output = Self;

    fn now() -> Self::Output {
        Self::new(chrono::Local::now().timestamp())
    }
}

impl<T> AsRef<T> for Timestamp<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Timestamp<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::borrow::Borrow<T> for Timestamp<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> core::borrow::BorrowMut<T> for Timestamp<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::ops::Deref for Timestamp<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Timestamp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> core::fmt::Display for Timestamp<T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Duration> for Timestamp<u64> {
    fn from(dur: Duration) -> Self {
        Self(dur.as_secs())
    }
}

impl From<Duration> for Timestamp<u128> {
    fn from(dur: Duration) -> Self {
        Self(dur.as_millis())
    }
}

#[cfg(feature = "time")]
impl<Tz> From<chrono::DateTime<Tz>> for Timestamp<i64>
where
    Tz: chrono::TimeZone,
{
    fn from(ts: chrono::DateTime<Tz>) -> Self {
        Self(ts.timestamp())
    }
}

impl From<Timestamp<u64>> for Duration {
    fn from(ts: Timestamp<u64>) -> Self {
        Self::from_secs(*ts)
    }
}

impl From<Timestamp<u128>> for Duration {
    fn from(ts: Timestamp<u128>) -> Self {
        Self::from_millis(*ts as u64)
    }
}
