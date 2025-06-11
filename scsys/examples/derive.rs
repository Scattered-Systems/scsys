/*
    Appellation: derive <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use scsys::time::Timestamp;

fn main() -> scsys::Result<()> {
    let mut unit = Unit::<usize>::new(42);
    println!("Unit: {unit}");
    let prev = unit.replace(100);
    println!("Unit: replaced {prev} with {unit}");
    let value = unit.take();
    println!("Taken unit: {value}");
    let u2 = unit.map(|x| (x + 1) * 2);
    println!("Unit value: {}", u2.get());

    let params = Sample::from_value(0.5);
    println!("Params: {params}");
    let variant = Something::a();
    println!("Variant: {:?}", variant);
    let variant = Something::b(42);
    println!("Variant: {:?}", variant);
    let variant = Something::c(1, 2);
    println!("Variant: {:?}", variant);
    Ok(())
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    scsys::Wrapper,
    scsys::Display,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case"),
    scsys(display(json))
)]
pub struct Unit<T>(pub T);

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, scsys::Getter, scsys::Display,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case"),
    scsys(display(json))
)]
pub struct Sample<T> {
    pub timestamp: Timestamp,
    pub value: T,
}

impl<T> Sample<T> {
    pub fn from_value(value: T) -> Self {
        let timestamp = Timestamp::now();
        Self { timestamp, value }
    }
}

#[derive(Clone, Copy, Debug, scsys::VariantConstructors)]
pub enum Something {
    A,
    B(usize),
    C { x: usize, y: usize },
}
