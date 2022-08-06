/*
    Appellation: generator <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use rand::distributions::{Distribution, Standard};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct StdGenerator<T>(T);

impl<T> StdGenerator<T>
where
    Standard: Distribution<T>,
{
    fn constructor(data: T) -> Result<Self, crate::BoxError> {
        Ok(Self(data))
    }
    pub fn new(data: T) -> Self {
        match Self::constructor(data) {
            Ok(v) => v,
            Err(e) => panic!("Generator Error: {}", e),
        }
    }
    pub fn random() -> Self {
        Self::new(crate::random_number::<T>())
    }
}

impl<T> Default for StdGenerator<T>
where
    Standard: Distribution<T>,
{
    fn default() -> Self {
        Self::new(crate::random_number::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let actual = StdGenerator::<f64>::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
