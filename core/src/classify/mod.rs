/*
    Appellation: classify <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::classifier::*;

mod classifier;

pub trait Classifiable<Cls: Classifier> {
    fn class(&self) -> &Cls;
}

pub trait Classify<Cls: Classifier> {
    fn classify(&self, target: Box<dyn Classifiable<Cls>>) -> Cls;
}
