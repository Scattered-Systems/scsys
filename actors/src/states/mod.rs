/*
    Appellation: states <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{specs::*, state::*};

pub(crate) mod state;

pub(crate) mod specs {
    use crate::messages::Message;
    use serde::Serialize;

    pub trait Stateful: Clone + Default + Serialize + std::fmt::Display {
        type Data: std::fmt::Display;

        fn message(&self) -> &Message<Self::Data>;
        fn timestamp(&self) -> i64;
    }

    pub trait StatefulExt: Stateful {
        fn agency(&self) -> String;
        fn catalyst<S, T>(&mut self, f: &dyn Fn(S) -> T) -> Vec<T>;
        fn tags(&self) -> Vec<String>;
    }

    pub trait StateTransition {
        type Dest: Stateful;
    }
}

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
