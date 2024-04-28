/*
    Appellation: params <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate scsys;

use scsys::Keyed;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _params = LinearParams { weight: 1.0 };
    let wk = LinearParamsKey::Weight;
    println!("{:?}", &wk);
    assert_eq!(LinearParamsKey::weight(), wk);
    // let _key = wk.key();
    Ok(())
}

#[derive(Keyed)]
pub struct LinearParams<T> {
    #[key]
    pub weight: T,
}
