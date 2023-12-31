/*
   Appellation: atomic <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::id::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AtomicId(usize);

impl AtomicId {
    pub fn new() -> Self {
        use std::sync::atomic;
        static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(1);
        Self(COUNTER.fetch_add(1, atomic::Ordering::Relaxed))
    }
}

impl AsRef<usize> for AtomicId {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl Default for AtomicId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AtomicId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl Identifier for AtomicId {}