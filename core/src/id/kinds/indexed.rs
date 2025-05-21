/*
    Appellation: id <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::id::Id;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IndexId<Idx = usize> {
    id: Id,
    index: Idx,
}

impl<Idx> IndexId<Idx> {
    pub fn new(id: Id, index: Idx) -> Self {
        Self { id, index }
    }
    pub fn from_index(index: Idx) -> Self {
        Self {
            id: Id::atomic(),
            index,
        }
    }

    pub fn id(&self) -> usize {
        *self.id
    }

    pub fn index(&self) -> &Idx {
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
