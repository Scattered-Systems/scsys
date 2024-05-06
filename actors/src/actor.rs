/*
    Appellation: actor <module>
    Creator: FL03 <jo3mccain@icloud.com>
*/

pub mod engine;

pub trait Actor {
    type Ctx;
    type Engine;
}
