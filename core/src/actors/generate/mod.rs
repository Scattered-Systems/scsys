/*
    Appellation: generate <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{alphabet::*, digits::*, utils::*};

pub(crate) mod alphabet;
pub(crate) mod digits;

pub(crate) mod utils {
    use rand::{
        distributions::{Alphanumeric, Standard},
        prelude::Distribution,
        Rng,
    };

    /// Consolidate resources required for generating random numbers
    pub fn generate_random_number<T>() -> T
        where
            Standard: Distribution<T>,
    {
        let mut rnd = rand::thread_rng();
        rnd.gen::<T>()
    }

    pub fn generate_random_string(len: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_num() {
        let r = DigitGenerator::<f64>::default();
        let s = DigitGenerator::<f64>::default();
        assert_ne!(r, s)
    }

    #[test]
    fn test_generator_str() {
        let actual = StringGenerator::default();
        let expected = StringGenerator::default();
        println!("{:#?}", actual.clone());
        assert_ne!(actual, expected)
    }

    #[test]
    fn test_random_number() {
        assert_ne!(
            generate_random_number::<f64>(),
            generate_random_number::<f64>()
        )
    }
}
