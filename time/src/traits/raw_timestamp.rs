/*
    appellation: timestamp <module>
    authors: @FL03
*/

/// A marker trait indicating types capable of representing a raw timestamp value.
pub trait RawTimestamp {
    private!();
}
/// The [`TimestampRepr`] trait provides a way of associating a raw timestamp type with its
/// corresponding value type.
pub trait TimestampRepr {
    type Value;

    private! {}
}

/*
 ************* Implementations *************
*/
use crate::timestamp::Timestamp;

impl<T> TimestampRepr for Timestamp<T>
where
    T: RawTimestamp,
{
    type Value = T;

    seal!();
}

impl<T> RawTimestamp for &T
where
    T: RawTimestamp + ?Sized,
{
    seal!();
}

impl<T> RawTimestamp for &mut T
where
    T: RawTimestamp + ?Sized,
{
    seal!();
}

macro_rules! impl_raw_timestamp {
    ($($t:ty),* $(,)?) => {
        $(
            impl_raw_timestamp!(@impl $t);
        )*
    };
    (@impl $T:ty) => {
        impl RawTimestamp for $T {
            seal!();
        }
    };
}

impl_raw_timestamp! {
    u64,
    u128,
    i64,
}

impl RawTimestamp for str {
    seal!();
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::RawTimestamp;
    use alloc::string::String;

    impl RawTimestamp for String {
        seal!();
    }
}

#[cfg(feature = "chrono")]
mod impl_chrono {
    use super::RawTimestamp;
    use chrono::DateTime;
    use chrono::offset::TimeZone;

    impl<Tz> RawTimestamp for DateTime<Tz>
    where
        Tz: TimeZone,
    {
        seal!();
    }
}
