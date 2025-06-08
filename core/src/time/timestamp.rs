/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::time::{Now, RawTimestamp};

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
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    /// create a new instance of [`Timestamp`] with the given value.
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
    pub const fn get(&self) -> &T {
        &self.0
    }
    /// Get a mutable reference to the current timestamp.
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    /// consumes the current instance and returns the inner value.
    #[inline]
    pub fn value(self) -> T {
        self.0
    }
    /// [`replace`](core::mem::replace) the current value with a new one and return the old one
    pub const fn replace(&mut self, value: T) -> T {
        core::mem::replace(self.get_mut(), value)
    }
    /// update the current value and return a mutable reference to the current instance.
    #[inline]
    pub fn set(&mut self, ts: T) -> &mut Self {
        *self.get_mut() = ts;
        self
    }
    /// [`swap`](core::mem::swap) the current value with another and return a mutable reference to self
    pub const fn swap(&mut self, ts: &mut T) {
        core::mem::swap(self.get_mut(), ts)
    }
    /// [`take`](core::mem::take) the current value and return it, leaving the logical
    /// [`default`](Default::default) in its place.
    #[inline]
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.get_mut())
    }
    /// consumes the current instance to create another with the given value
    #[inline]
    pub fn with<U: RawTimestamp>(self, ts: U) -> Timestamp<U> {
        Timestamp(ts)
    }
    /// applies a function onto the current value and returns a new instance with the result
    pub fn map<U, F>(self, f: F) -> Timestamp<U>
    where
        F: FnOnce(T) -> U,
        U: RawTimestamp,
    {
        Timestamp(f(self.value()))
    }
    /// returns a new instance of the [`Timestamp`] with the current value updated using the given function
    pub fn map_inplace<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut T),
    {
        f(self.get_mut());
        self
    }
    /// updates the timestamp to reflect _now_ and return the previous timestamp
    pub fn update(&mut self) -> T
    where
        T: Now<Output = T>,
    {
        let now = T::now();
        // replace the current value with the new one
        self.replace(now)
    }
    /// returns a new instance of the [`Timestamp`] containing an immutable reference to the
    /// inner value.
    pub const fn view(&self) -> Timestamp<&T> {
        Timestamp(self.get())
    }
    /// returns a new instance of the [`Timestamp`] containing a mutable reference to the inner
    /// value.
    pub const fn view_mut(&mut self) -> Timestamp<&mut T> {
        Timestamp(self.get_mut())
    }
}
#[allow(deprecated)]
impl<T> Timestamp<T>
where
    T: RawTimestamp,
{
    #[deprecated(since = "0.2.8", note = "use `Timestamp::get` instead")]
    pub fn as_ref(&self) -> &T {
        self.get()
    }
    #[deprecated(since = "0.2.8", note = "use `Timestamp::get_mut` instead")]
    pub fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
    #[deprecated(since = "0.2.8", note = "use `Timestamp::value` instead")]
    pub fn into_inner(self) -> T {
        self.0
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
