/*
    Appellation: impl_std <module>
    Created At: 2025.09.08:17:30:03
    Contrib: @FL03
*/
#[allow(unused_imports)]
use core::time::Duration;

/// the [`systime`] function returns the current system timestamp as a [`Duration`] allowing
/// users to convert the object into their desired units.
#[inline]
pub fn systime() -> Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
}
/// the [`std_time`] function returns the current system time in milliseconds as an [`u128`].
#[inline]
pub fn std_time() -> u128 {
    systime().as_millis()
}
