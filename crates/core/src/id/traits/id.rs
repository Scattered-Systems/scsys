pub trait RawIdentifier: Send + Sync + core::fmt::Debug + core::fmt::Display {
    private!();
}

pub trait StdId: RawIdentifier {
    /// Returns the raw identifier as a byte slice.
    fn as_bytes(&self) -> &[u8];
}

/*
 ************* Implementations *************
*/
#[cfg(feature = "alloc")]
use alloc::string::String;

macro_rules! impl_raw_id {
    (@impl $t:ty) => {
        impl RawIdentifier for $t {
            seal!();
        }
    };
    ($($t:ty),* $(,)?) => {
        $(
            impl_raw_id!(@impl $t);
        )*
    };
}

impl_raw_id! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64, bool, char, str,
}

#[cfg(feature = "alloc")]
impl_raw_id! {
    String
}
