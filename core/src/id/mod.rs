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

    #[cfg(feature = "rand")]
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

pub(crate) mod prelude {
    pub use super::ids::*;
    pub use super::utils::*;
    pub use super::Identifiable;
    pub use super::Identifier;
}

#[cfg(test)]
mod tests {
    use super::ids::AtomicId;

    #[test]
    fn test_atomic() {
        let a = AtomicId::new();
        assert_eq!(*a.next(), *a + 1);
        let b = AtomicId::new();
        assert_eq!(*b, *a + 2);
    }
    #[cfg(feature = "rand")]
    #[test]
    fn test_rid() {
        let id = super::rid(10);
        assert_eq!(id.len(), 10);
    }
}
