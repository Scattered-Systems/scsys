/*
    Appellation: agency <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub trait Agency: ToString {
    fn new() -> Self;
    fn agent(&self) -> String;
}
