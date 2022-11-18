/*
    Appellation: configurable <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::Serialize;

pub trait Configurable: Serialize {
    type Settings;

    fn settings(&self) -> &Self::Settings;
}
