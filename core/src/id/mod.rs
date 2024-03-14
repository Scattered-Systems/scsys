/*
    Appellation: identity <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # identity
pub use self::{identity::*, utils::*};

pub(crate) mod identity;

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
        use rand::Rng;

        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::ids::AtomicId;
    use super::rid;

    #[test]
    fn test_atomic() {
        let id = AtomicId::new();
        assert_eq!(id.get(), 1);
        assert_eq!(*id.next(), 2);
        let id = AtomicId::new();
        assert_eq!(id.get(), 3);
    }

    #[test]
    fn test_rid() {
        let id = rid(10);
        assert_eq!(id.len(), 10);
    }
}
