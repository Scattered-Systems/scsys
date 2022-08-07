/*
    Appellation: generator <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use rand::{distributions, prelude::Distribution};

#[derive(Clone, Copy, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DigitGenerator<T>(T);

impl<T> DigitGenerator<T> {
    fn constructor(data: T) -> crate::BoxResult<Self> {
        Ok(Self(data))
    }
    pub fn new(data: T) -> Self {
        match Self::constructor(data) {
            Ok(v) => v,
            Err(e) => panic!("Generator Error: {}", e),
        }
    }
}

impl<T> DigitGenerator<T>
    where
        distributions::Standard: Distribution<T>,
{
    pub fn generate() -> Self {
        Self::new(crate::generate::generate_random_number::<T>())
    }
}

impl<T> Default for DigitGenerator<T>
    where
        distributions::Standard: Distribution<T>,
{
    fn default() -> Self {
        Self::generate()
    }
}
