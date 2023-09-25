/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::distributions::{uniform, Alphanumeric, Distribution, Standard};
use rand::Rng;

///
pub fn generate_random_number<T>() -> T
where
    Standard: Distribution<T>,
{
    rand::thread_rng().gen::<T>()
}

///
pub fn generate_random_number_between<T, R>(range: R) -> T
where
    T: uniform::SampleUniform,
    R: uniform::SampleRange<T>,
{
    rand::thread_rng().gen_range::<T, R>(range)
}

///
pub fn generate_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
