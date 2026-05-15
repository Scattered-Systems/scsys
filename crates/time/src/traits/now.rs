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

#[cfg(all(feature = "alloc", feature = "chrono"))]
mod impl_alloc {
    use super::Now;
    use crate::Timestamp;
    use alloc::string::String;

    impl Now for String {
        type Output = Timestamp<Self>;

        fn now() -> Self::Output {
            Timestamp::new(chrono::Local::now().to_rfc3339())
        }
    }
}

#[cfg(feature = "chrono")]
mod impl_chrono {
    use super::Now;
    use crate::timestamp::Timestamp;

    impl Now for i64 {
        type Output = Timestamp<Self>;

        fn now() -> Self::Output {
            Timestamp::new(chrono::Local::now().timestamp())
        }
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::Now;
    use crate::timestamp::Timestamp;
    use crate::utils::systime;

    impl Now for u128 {
        type Output = Timestamp<Self>;

        fn now() -> Self::Output {
            Timestamp::new(systime().as_millis())
        }
    }

    impl Now for u64 {
        type Output = Timestamp<Self>;

        fn now() -> Self::Output {
            Timestamp::new(systime().as_secs())
        }
    }
}
