/*
    appellation: timestamp <module>
    authors: @FL03
*/

/// a private trait used to mark types capable of being uses as a basetype for a [`Timestamp`].
pub trait RawTimestamp {
    private!();
}

/*
 ************* Implementations *************
*/
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
impl RawTimestamp for alloc::string::String {
    seal!();
}
