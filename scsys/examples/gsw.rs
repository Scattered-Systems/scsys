/*
    Appellation: derive <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use scsys::time::Timestamp;

fn main() -> scsys::Result<()> {
    let params = Sample::from_value(0.5);
    println!("Timestamp: {}", params.timestamp());
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
    scsys::Display,
    scsys::Get,
    scsys::GetMut,
    scsys::Set,
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
