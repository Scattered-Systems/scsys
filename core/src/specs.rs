/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Appellation;

pub trait Addressable<T> {
    fn address(&self) -> T;
}

pub trait InputName {
    fn name(&self) -> String;
    fn slug(&self) -> String {
        self.name().to_lowercase()
    }
}

pub trait ActorSpec<T> {
    fn appellation(&self) -> Appellation<T>;
    fn justification(&self) -> serde_json::Value;
}

/// Quickly derive elligible naming schematics for the desired structure
pub trait Named {
    fn name() -> String;
    fn slug(&self) -> String {
        Self::name().to_lowercase()
    }
}
