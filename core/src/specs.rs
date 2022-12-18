/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub trait Addressable<T> {
    fn address(&self) -> T;
}
/// Trait for signaling a structure with a dedicated build stage
pub trait Buildable {
    type Error;
    fn build() -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// Quickly derive elligible naming schematics for the desired structure
pub trait Named {
    fn name() -> String;
    fn slug(&self) -> String {
        Self::name().to_lowercase()
    }
}
