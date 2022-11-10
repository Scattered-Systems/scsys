/*
    Appellation: configurable <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub trait Configurable {
    type Settings;

    fn settings(&self) -> &Self::Settings;
}
