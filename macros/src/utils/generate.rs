/*
    Appellation: generate <utils>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use rand::{
    distributions::{Alphanumeric, DistString, Distribution, Standard},
    Rng,
};

pub fn gen_random_num<T>() -> T
where
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn gen_random_string(size: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), size)
}
