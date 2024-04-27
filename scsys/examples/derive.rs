/*
    Appellation: derive <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate scsys;

use scsys::{Keyed, VariantConstructors};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _params = LinearParams { weight: 1.0 };
    let wk = LinearParamsKey::Weight;
    println!("{:?}", &wk);
    println!("{:?}", Something::a());
    assert_eq!(LinearParamsKey::weight(), wk);
    // let _key = wk.key();
    Ok(())
}

#[derive(Keyed)]
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
