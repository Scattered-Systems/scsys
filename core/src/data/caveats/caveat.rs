/*
    Appellation: caveat <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde_json::Value;

pub trait Caveat {
    fn caveat(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
}

pub struct Justification {
    pub caveats: Vec<Box<dyn Caveat>>,
}
