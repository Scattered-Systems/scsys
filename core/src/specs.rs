/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub trait Addressable {
    type Addr;
    fn address(self) -> Self::Addr;
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
    fn name(&self) -> String;
    fn slug(&self) -> String {
        self.name().to_lowercase()
    }
}
