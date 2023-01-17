/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use rand::{distributions::{Alphanumeric, Standard}, prelude::Distribution, Rng};

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

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_random_number() {
        assert_ne!(
            generate_random_number::<f64>(),
            generate_random_number::<f64>()
        )
    }
}