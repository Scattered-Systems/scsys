/*
    Appellation: params <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate scsys;

use scsys::Params;

fn main() -> scsys::prelude::Result<()> {
    let _params = LinearParams { weight: 1.0 };
    let wk = LinearParamsKey::Weight;
    println!("{:?}", &wk);
    assert_eq!(LinearParamsKey::weight(), wk);
    // let _key = wk.key();
    Ok(())
}

#[derive(Params)]
pub struct LinearParams<T> {
    #[param]
    pub weight: T,
}
