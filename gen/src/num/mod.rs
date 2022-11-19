/*
    Appellation: num <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{digits::*, utils::*};

pub(crate) mod digits;

pub(crate) mod utils {
    use rand::{distributions::Standard, prelude::Distribution, Rng};

    /// Consolidate resources required for generating random numbers
    pub fn generate_random_number<T>() -> T
    where
        Standard: Distribution<T>,
    {
        let mut rnd = rand::thread_rng();
        rnd.gen::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_number_default() {
        assert_ne!(
            DigitGenerator::<f64>::default(),
            DigitGenerator::<f64>::default()
        )
    }

    #[test]
    fn test_random_number() {
        assert_ne!(
            generate_random_number::<f64>(),
            generate_random_number::<f64>()
        )
    }
}
