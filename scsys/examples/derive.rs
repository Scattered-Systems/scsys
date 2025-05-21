/*
    Appellation: derive <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use scsys::prelude::Result;

fn main() -> Result<()> {
    let params = LinearParams { weight: 0.5 };
    println!("Params: {}", params);
    let variant = Something::a();
    println!("Variant: {:?}", variant);
    let variant = Something::b(42);
    println!("Variant: {:?}", variant);
    let variant = Something::c(1, 2);
    println!("Variant: {:?}", variant);
    Ok(())
}

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, scsys::Getter, scsys::Display,
)]
#[scsys(display(serde))]
pub struct LinearParams<T> {
    pub weight: T,
}

#[derive(Clone, Copy, Debug, scsys::VariantConstructors)]
pub enum Something {
    A,
    B(usize),
    C { x: usize, y: usize },
}
