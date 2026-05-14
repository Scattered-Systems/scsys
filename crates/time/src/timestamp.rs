/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
mod impl_timestamp;
mod impl_timestamp_repr;

#[allow(deprecated)]
mod impl_deprecated;

/// [`Timestamp`] is a generic implementation of a type that represents some point in time.
///
/// ## Basic Usage
///
/// ```rust
/// use scsys_time::Timestamp;
///
/// let ts = Timestamp::<u128>::now();
/// println!("Current Timestamp: {}", ts);
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
pub struct Timestamp<T = u128>(pub T);

/*
 ************* Implementations *************
*/

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
