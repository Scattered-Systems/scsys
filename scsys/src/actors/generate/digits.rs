/*
    Appellation: digit <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use rand::{distributions, prelude::Distribution};

#[derive(Clone, Copy, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DigitGenerator<T>(T);

impl<T> DigitGenerator<T> {
    fn constructor(data: T) -> Self {
        Self(data)
    }
    pub fn new(data: T) -> Self {
        Self::constructor(data)
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

#[cfg(test)]
mod tests {
    use super::DigitGenerator;

    #[test]
    fn test_generator_num() {
        assert_ne!(
            DigitGenerator::<f64>::default(),
            DigitGenerator::<f64>::default()
        )
    }
}
