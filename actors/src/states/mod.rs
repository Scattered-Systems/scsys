/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # States
pub use self::state::*;

mod state;

pub(crate) mod specs {
    use std::ops::MulAssign;
    use std::sync::{Arc, Mutex};

    /// [AsyncStateful] describes an async stateful object
    pub trait AsyncStateful<S: StateSpec>: Clone {
        fn state(&self) -> Arc<Mutex<S>>;
        fn update_state(&mut self, state: Arc<Mutex<S>>);
    }

    /// [Stateful] describes a stateful object
    pub trait Stateful<S: StateSpec>: Clone {
        /// [Stateful::state] is used to get the state of the object
        fn state(&self) -> S;
        /// [Stateful::update_state] is used to update the state of the object
        fn update_state(&mut self, state: S);
    }

    impl Stateful<i64> for i64 {
        fn state(&self) -> i64 {
            *self
        }
        fn update_state(&mut self, state: i64) {
            *self = state;
        }
    }

    /// [StateSpec] is used by [Stateful] to describe a specific state
    pub trait StateSpec: Clone + Default + Eq + Ord + ToString + MulAssign {}

    impl<T> StateSpec for T where T: Clone + Default + Eq + Ord + ToString + MulAssign {}
}
