/*
    appellation: now <module>
    authors: @FL03
*/

/// The [`Now`] trait provides a common creation routines for all datetime implementations.
pub trait Now {
    type Output;

    fn now() -> Self::Output;
}

/*
 ************* Implementations *************
*/
#[cfg(any(feature = "chrono", feature = "std"))]
use crate::time::Timestamp;
#[cfg(feature = "std")]
use crate::time::utils::systime;
#[cfg(all(feature = "alloc", feature = "chrono"))]
use alloc::string::String;

#[cfg(feature = "std")]
impl Now for u64 {
    type Output = Timestamp<Self>;

    fn now() -> Self::Output {
        Timestamp::new(systime().as_secs())
    }
}

#[cfg(feature = "std")]
impl Now for u128 {
    type Output = Timestamp<Self>;

    fn now() -> Self::Output {
        Timestamp::new(systime().as_millis())
    }
}

#[cfg(feature = "chrono")]
impl Now for i64 {
    type Output = Timestamp<Self>;

    fn now() -> Self::Output {
        Timestamp::new(chrono::Local::now().timestamp())
    }
}

#[cfg(all(feature = "alloc", feature = "chrono"))]
impl Now for String {
    type Output = Timestamp<Self>;

    fn now() -> Self::Output {
        Timestamp::new(chrono::Local::now().to_rfc3339())
    }
}
