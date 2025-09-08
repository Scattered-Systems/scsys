/*
    Appellation: impl_std <module>
    Created At: 2025.09.08:17:30:03
    Contrib: @FL03
*/
#[allow(unused_imports)]

/// [systime] is a utilitarian function that returns the current system time in milliseconds.
#[inline]
pub fn systime() -> core::time::Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
}
/// [systime] is a utilitarian function that returns the current system time in milliseconds.
#[inline]
pub fn std_time() -> u128 {
    systime().as_millis()
}
