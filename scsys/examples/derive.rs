/*
    Appellation: derive <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate scsys;

use scsys::{Keyed, VariantConstructors};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let params = LinearParams { weight: 1.0 };
    println!("{:?}", &params);
    let wk = LinearParamsKey::Weight;
    println!("{:?}", &wk);
    println!("{:?}", Something::a());
    assert_eq!(LinearParamsKey::weight(), wk);
    // let _key = wk.key();
    Ok(())
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Keyed, Ord, PartialEq, PartialOrd)]
pub struct LinearParams<T> {
    #[key]
    pub weight: T,
}

#[derive(Clone, Copy, Debug, VariantConstructors)]
pub enum Something {
    A,
    B(usize),
    C { x: usize, y: usize },
}
