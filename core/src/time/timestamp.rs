/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::time::{Now, RawTimestamp};
use core::time::Duration;

/// [`Timestamp`] is a generic implementation of a type that represents some point in time.
///
/// ## Usage
///
/// ### Examples
///
/// #### _Example #1_ Using the [`now`](Timestamp::now) method to get the current timestamp
///
/// ```rust
/// #[cfg(feature = "std")]
/// let ts = scsys_core::Timestamp::<u64>::now();
/// ```
///
/// ## Features
///
/// The timestamps implementation dynamically reflects the extensive feature-gating of the
/// crate. Listed below are the features that customize the behavior of the [`Timestamp`] type:
///
/// - `serde`: Enables serialization and deserialization of the [`Timestamp`] type.
///
/// - `chrono`: Enables support for `chrono` timestamps, uses [`i64`] for the [Now] impl
/// - `std`: Enables the use of system time for the default implementation of:
///   - `Timestamp<u64>`: for seconds
///   - `Timestamp<u128>`: for milliseconds
///
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
#[repr(transparent)]
pub struct Timestamp<T = u64>(pub T);

impl<T> Timestamp<T>
where
    T: RawTimestamp,
{
    pub fn new(ts: T) -> Self {
        Self(ts)
    }
    /// a convenience method to create a new [`Timestamp`] instance with the current time.
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
    pub fn into_inner(self) -> T {
        self.0
    }
    /// [`replace`](core::mem::replace) the current value with a new one and return the old one
    pub fn replace(&mut self, value: T) -> T {
        core::mem::replace(self.as_mut(), value)
    }
    /// set the current state and return a mutable reference to sel
    pub fn set(&mut self, ts: T) -> &mut Self {
        self.0 = ts;
        self
    }
    /// [`take`](core::mem::take) the current value and return it, leaving the logical
    /// [`default`](Default::default) in its place.
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.as_mut())
    }
    /// consumes the current instance to create another with the given value
    pub fn with<U: RawTimestamp>(self, ts: U) -> Timestamp<U> {
        Timestamp(ts)
    }
}

impl<T> Timestamp<T>
where
    T: RawTimestamp,
{
    #[deprecated(since = "0.2.8", note = "use `Timestamp::into_inner` instead")]
    pub fn get(self) -> T {
        self.into_inner()
    }
    #[doc(hidden)]
    #[deprecated(since = "0.2.8", note = "use `Timestamp::set` instead")]
    pub fn update(&mut self, ts: T) {
        self.0 = ts;
    }
}

impl<T> Default for Timestamp<T>
where
    Self: Now<Output = Self>,
    T: RawTimestamp,
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

impl<T: RawTimestamp> AsRef<T> for Timestamp<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: RawTimestamp> AsMut<T> for Timestamp<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: RawTimestamp> core::borrow::Borrow<T> for Timestamp<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T: RawTimestamp> core::borrow::BorrowMut<T> for Timestamp<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: RawTimestamp> core::ops::Deref for Timestamp<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: RawTimestamp> core::ops::DerefMut for Timestamp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> core::fmt::Display for Timestamp<T>
where
    T: RawTimestamp + core::fmt::Display,
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

#[cfg(feature = "chrono")]
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
