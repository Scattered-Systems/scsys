/*
    Appellation: classify <specs>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Classify
//!
//!

pub trait Classifiable<Cls: Classifier> {
    fn class(&self) -> &Cls;
}

pub trait Classify<Cls: Classifier> {
    fn classify(&self, target: Box<dyn Classifiable<Cls>>) -> Cls;
}

/// Interface for classifier objects
pub trait Classifier {}

impl Classifier for i8 {}

impl Classifier for i32 {}

impl Classifier for i64 {}

impl Classifier for i128 {}

impl Classifier for isize {}

impl Classifier for u8 {}

impl Classifier for u32 {}

impl Classifier for u64 {}

impl Classifier for u128 {}

impl Classifier for usize {}

impl Classifier for f32 {}

impl Classifier for f64 {}

impl Classifier for String {}

impl Classifier for &str {}

impl Classifier for char {}
