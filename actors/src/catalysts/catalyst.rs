/*
    Appellation: catalyst <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::messages::Message;
use crate::Direction;

pub struct Catalyzer {
    pub direction: Direction,
    pub message: Message,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_default() {}
}
