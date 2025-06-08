/*
    Appellation: id <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::id::Id;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IndexId<T, Idx = usize> {
    id: Id<T>,
    index: Idx,
}

impl<T, Idx> IndexId<T, Idx> {
    pub fn new(id: Id<T>, index: Idx) -> Self {
        Self { id, index }
    }
    /// returns an immutable reference to the id
    pub const fn id(&self) -> &Id<T> {
        &self.id
    }
    /// returns an immutable reference to the index
    pub const fn index(&self) -> &Idx {
        &self.index
    }
}

impl<Idx> core::fmt::Display for IndexId<Idx>
where
    Idx: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}({})", self.id(), self.index())
    }
}
