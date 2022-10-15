/*
    Appellation: digit <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::generate_random_number;
use rand::{distributions, prelude::Distribution};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DigitGenerator<T>(T);

impl<T> DigitGenerator<T> {
    pub fn new(data: T) -> Self {
        Self(data)
    }
}

impl<T> DigitGenerator<T>
where
    distributions::Standard: Distribution<T>,
{
    pub fn generate() -> Self {
        Self::new(generate_random_number::<T>())
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
