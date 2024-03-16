/*
    Appellation: message <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use scsys::prelude::{AtomicId, Timestamp};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Message<T = String> {
    id: AtomicId,
    data: Option<T>,
    ts: Timestamp,
}

impl<T> Message<T> {
    pub fn new(data: Option<T>) -> Self {
        Self {
            id: AtomicId::new(),
            data,
            ts: Timestamp::now(),
        }
    }

    pub fn content(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub fn id(&self) -> usize {
        *self.id
    }

    pub fn timestamp(&self) -> Timestamp {
        self.ts
    }
}

impl<T> std::fmt::Display for Message<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(data) = self.content() {
            return write!(f, "Timestamp: {}\n{}", self.ts, data,);
        }
        write!(f, "Timestamp: {}", self.ts)
    }
}

impl<T> From<Option<T>> for Message<T> {
    fn from(data: Option<T>) -> Self {
        Self::new(data)
    }
}

impl<T> From<T> for Message<T> {
    fn from(data: T) -> Self {
        Self::new(Some(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_message() {
        let a = Message::<String>::default();
        assert_eq!(&a.data, &a.data)
    }
}
