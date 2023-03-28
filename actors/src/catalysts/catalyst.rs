/*
    Appellation: catalyst <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{messages::Message, Direction};
use decanter::{crypto::Hashable, Hash};
use serde::{Deserialize, Serialize};

pub struct Catalyzer {
    pub direction: Direction,
    pub message: Message,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {}
}
