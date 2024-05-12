/*
    Appellation: classify <specs>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(all(feature = "alloc", no_std))]
use alloc::string::String;

/// Interface for classifiable objects
pub trait Classifiable {
    type Class: Classifier;

    fn class(&self) -> &Self::Class;
}

/// Denotes an object that may be used as a classifier
pub trait Classifier {}

macro_rules! classifier {
    ($($t:ty),*) => {
        $(
            classifier!(@loop $t);
        )*
    };
    (@loop $t:ty) => {
        impl Classifier for $t {}
    };
}

classifier!(f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, &str, char);

#[cfg(any(feature = "alloc", feature = "std"))]
classifier!(String);
