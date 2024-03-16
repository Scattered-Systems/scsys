/*
    Appellation: identity <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::ids::AtomicId;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;

pub enum TypeClass {
    Alphanumeric,
    Numeric,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
pub struct Id<T = AtomicId> {
    id: T,
    timestamp: u128,
}

impl<T> Id<T> {
    pub fn new(id: T) -> Self {
        Self {
            id,
            timestamp: crate::time::systime(),
        }
    }

    pub fn id(&self) -> &T {
        &self.id
    }

    pub fn set(&mut self, id: T) {
        self.id = id;
        self.on_update();
    }

    pub fn timestamp(&self) -> u128 {
        self.timestamp
    }

    fn on_update(&mut self) {
        self.timestamp = crate::time::systime();
    }
}

impl<T> AsRef<T> for Id<T> {
    fn as_ref(&self) -> &T {
        &self.id
    }
}

impl<T> Deref for Id<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
