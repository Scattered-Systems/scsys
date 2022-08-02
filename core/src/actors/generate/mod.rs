/*
    Appellation: generate <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use generator::*;

pub(crate) mod generator;

///
pub trait GenNumberSpec {
    fn random_number<T>() -> T
        where
            rand::distributions::Standard: rand::prelude::Distribution<T>,
    {
        use rand::Rng;
        rand::thread_rng().gen::<T>()
    }
}
