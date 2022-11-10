/*
    Appellation: justifiable <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde_json::Value;

pub trait Justifiable {
    fn justification(&self) -> &Value;
}