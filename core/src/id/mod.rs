/*
    Appellation: identity <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # identity
pub use self::{builder::*, identity::*, kinds::*, utils::*};

pub(crate) mod builder;
pub(crate) mod identity;
pub(crate) mod kinds;

pub mod ids;

pub type BoxedId = Box<dyn Identifier>;

/// Interface for identifiable data-structures
pub trait Identifiable {
    type Id: Identifier;

    fn id(&self) -> &Self::Id;
}

impl<Id: Identifier> Identifiable for Id {
    type Id = Id;

    fn id(&self) -> &Self::Id {
        self
    }
}

/// Interface for identifier data-structures
pub trait Identifier: ToString {}

// impl<T> Identifier for T where T: ToString {}

pub(crate) mod utils {
    // https://users.rust-lang.org/t/idiomatic-rust-way-to-generate-unique-id/33805
    pub fn atomic_id() -> usize {
        use std::sync::atomic;
        static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
        COUNTER.fetch_add(1, atomic::Ordering::Relaxed)
    }

    pub fn rid(length: usize) -> String {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_atomic_id() {
        let id = atomic_id();
        assert_eq!(id, 0);
        assert_ne!(id, atomic_id());
    }

    #[test]
    fn test_rid() {
        let id = rid(10);
        assert_eq!(id.len(), 10);
    }
}
