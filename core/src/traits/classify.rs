/*
    Appellation: classify <specs>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// Interface for classifiable objects
pub trait Classifiable {
    type Class: Classifier;

    fn class(&self) -> &Self::Class;
}

/// Denotes an object that may be used as a classifier
pub trait Classifier {}

macro_rules! impl_classifier {
    ($($t:ty),*) => {
        $(
            impl_classifier!(@loop $t);
        )*
    };
    (@loop $t:ty) => {
        impl Classifier for $t {}
    };
}

impl_classifier!(
    f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, &str, String, char
);
