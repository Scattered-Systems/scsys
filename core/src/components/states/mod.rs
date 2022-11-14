/*
    Appellation: states <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{state::*, stateful::*};

pub(crate) mod state;
pub(crate) mod stateful;

#[cfg(test)]
mod tests {
    use super::State;

    #[test]
    fn test_state_default() {
        let actual = State::from("test");
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
