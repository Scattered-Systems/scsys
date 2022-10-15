/*
    Appellation: states <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::state::State;

pub(crate) mod state;

pub(crate) mod primitives {
    use crate::{crypto::hashes, Dictionary};

    pub type StateHashMap<K = hashes::H160, V = (usize, usize)> = Dictionary<K, V>;
}

pub trait Stateful<T, S = crate::StateHashMap> {
    fn state(&self) -> S;
    fn transition(&self, state: S, f: dyn Fn(S) -> T) -> T;
}
