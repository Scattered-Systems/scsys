/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{kinds::*, traits::*};

pub(crate) mod traits;

mod kinds {
    pub use self::{atomic::AtomicId, indexed::IndexId};

    pub mod atomic;
    pub mod indexed;
}

pub(crate) mod prelude {
    pub use super::kinds::*;
    pub use super::traits::*;
}

#[cfg(test)]
mod tests {
    use super::traits::*;
    use super::AtomicId;

    #[test]
    fn test_id() {
        let id = 0usize.get();
        assert_eq!(id, &0);
        let atomic = AtomicId::new();
        let aid = Id::<usize>::get(&atomic);
        assert_ne!(*aid, *id);
    }
}
