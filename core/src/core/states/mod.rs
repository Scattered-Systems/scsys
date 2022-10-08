/*
    Appellation: states <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::state::State;

pub(crate) mod state;


pub type StateHashMap<T = crate::crypto::hash::H160> = std::collections::HashMap<T, (usize, usize)>;

pub trait Stateful<T, S = StateHashMap> {
    fn state(&self) -> S;
    fn transition(&self, state: S, f: dyn Fn(S) -> T) -> T;
}

