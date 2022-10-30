/*
    Appellation: states <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::state::State;

pub(crate) mod state;

pub trait Stateful<Msg: ToString = String>: Clone + ToString {
    fn message(&self) -> &Msg;
    fn state(&self) -> &Self {
        self
    }
    fn timestamp(&self) -> i64;
}

#[cfg(test)]
mod tests {
    use super::State;

    #[test]
    fn test_state_default() {
        let actual = State::new("message".to_string(), "test");
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
