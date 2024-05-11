/*
    Appellation: message <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use scsys::prelude::{AtomicId, Timestamp};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Message<T = String> {
    id: AtomicId,
    data: Option<T>,
    ts: Timestamp,
}

impl<T> Message<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: AtomicId::new(),
            data: Some(value),
            ts: Timestamp::now(),
        }
    }

    pub fn clear(&mut self) {
        self.on_update();
        self.data = None;
    }

    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub fn data_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }

    pub fn id(&self) -> usize {
        *self.id
    }

    pub fn set_data(&mut self, data: Option<T>) {
        self.on_update();
        self.data = data;
    }

    pub fn timestamp(&self) -> Timestamp {
        self.ts
    }

    pub fn with_data(mut self, data: T) -> Self {
        self.on_update();
        self.data = Some(data);
        self
    }

    fn on_update(&mut self) {
        self.ts = Timestamp::now();
    }
}

impl<T> Default for Message<T> {
    fn default() -> Self {
        Self {
            id: AtomicId::new(),
            data: None,
            ts: Timestamp::now(),
        }
    }
}
impl<T> core::fmt::Display for Message<T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(data) = self.data() {
            return write!(f, "{}", data);
        }
        write!(f, "{}", self.timestamp())
    }
}

impl<T> From<Option<T>> for Message<T> {
    fn from(data: Option<T>) -> Self {
        if let Some(msg) = data {
            return Self::new(msg);
        }
        Self::default()
    }
}

impl<T> From<T> for Message<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_message() {
        let a = Message::<&str>::default();
        assert_eq!(&a.data, &a.data)
    }
}
