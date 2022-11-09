/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub trait Configurable {
    fn settings(&self) -> &Self {
        self
    }
}

pub trait Contextual: Configurable {
    fn context(&self) -> &Self {
        self
    }
}
