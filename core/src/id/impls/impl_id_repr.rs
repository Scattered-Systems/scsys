/*
    appellation: impl_id_repr <module>
    authors: @FL03
*/
use crate::id::Id;

use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
#[cfg(feature = "uuid")]
use uuid::Uuid;
/// the static counter used for the [`Id<usize>`] representation
static COUNTER: AtomicUsize = AtomicUsize::new(0);

impl<T> Id<&T> {
    /// returns a new identifier with a cloned inner value
    pub fn cloned(&self) -> Id<T>
    where
        T: Clone,
    {
        Id(self.0.clone())
    }
    /// returns a new identifier with the inner value copied
    pub fn copied(&self) -> Id<T>
    where
        T: Copy,
    {
        Id(*self.0)
    }
}

impl<T> Id<&mut T> {
    /// returns a new identifier with a cloned inner value
    pub fn cloned(&self) -> Id<T>
    where
        T: Clone,
    {
        Id(self.0.clone())
    }
    /// returns a new identifier with the inner value copied
    pub fn copied(&self) -> Id<T>
    where
        T: Copy,
    {
        Id(*self.0)
    }
}

impl Id<usize> {
    /// initializes a new identifie depending on the available features; more specifically,
    /// whenever the `rand` feature is enabled, it will create a random identifier, otherwise
    /// it will default to an atomic counter.
    pub fn create() -> Self {
        let value: usize;
        #[cfg(feature = "rand")]
        {
            value = rand::random::<u128>() as usize;
        }
        #[cfg(not(feature = "rand"))]
        {
            value = COUNTER.fetch_add(1, Relaxed);
        }
        Id::new(value)
    }
    pub fn atomic() -> Self {
        Self::new(COUNTER.fetch_add(1, Relaxed))
    }
    /// replaces the current id with the atomic-ally next value and returns the previous value.
    /// see [`step`](Id::step) for more information
    pub fn atomic_step(&mut self) -> usize {
        self.replace(COUNTER.fetch_add(1, Relaxed))
    }
}

#[cfg(feature = "uuid")]
impl Id<Uuid> {
    pub fn v3(namespace: &Uuid, name: &[u8]) -> Self {
        let id = Uuid::new_v3(namespace, name);
        Self::new(id)
    }

    #[cfg(all(feature = "rng", feature = "uuid"))]
    pub fn v4() -> Self {
        let id = Uuid::new_v4();
        Self::new(id)
    }
}
