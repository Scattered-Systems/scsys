/*
    Appellation: data <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use utils::*;

pub mod handlers;
pub mod models;
pub mod schemas;

mod utils {
    pub struct Extract<Dt>
        where
            Dt: Clone + std::str::FromStr,
            <Dt as std::str::FromStr>::Err: std::fmt::Debug,
    {
        pub data: Vec<Dt>,
    }
}
