/*
    Appellation: derive <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use scsys::prelude::Result;

fn main() -> Result<()> {
    let params = LinearParams { weight: 1.0 };
    println!("{:?}", params.weight());
    let wk = LinearParamsKey::Weight;
    println!("{:?}", &wk);
    println!("{:?}", Something::a());
    assert_eq!(LinearParamsKey::weight(), wk);
    // let _key = wk.key();
    Ok(())
}

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, scsys::Getter, scsys::Params,
)]
pub struct LinearParams<T> {
    #[param]
    pub weight: T,
}

#[derive(Clone, Copy, Debug, scsys::VariantConstructors)]
pub enum Something {
    A,
    B(usize),
    C { x: usize, y: usize },
}
