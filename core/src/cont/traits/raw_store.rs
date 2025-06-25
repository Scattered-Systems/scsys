/*
    appellation: store <module>
    authors: @FL03
*/
use super::{RawContainer, RawEntry};

/// [`RawStore`] is a trait extending the [`RawContainer`] trait to establish a common
/// interface for all key-value stores.
pub trait RawStore<K, V>: RawContainer<Item = V> {
    private!();
}
/// the [`KeyValue`] trait extends the [`RawKeyValue`] trait to provide additional methods for
/// manipulating key-value stores.
pub trait Store<K, V>: RawStore<K, V> {
    /// the entry for the key-value pair
    type Entry<'a>: RawEntry<'a, Key = K, Value = V>
    where
        Self: 'a;
    /// returns the [`Entry`] for the given key
    fn entry(&mut self, key: K) -> Self::Entry<'_>;

    fn insert(&mut self, key: K, value: V) -> Option<V>;
}
/// The [`StoreIter`] trait extends the [`RawStore`] trait to provide iteration capabilities
/// over the vertices stored in the edge.
pub trait StoreIter<K, V> {
    type Item<'a, _K, _V>
    where
        Self: 'a;
    /// the iterator for the store
    type Iter<'a>: Iterator<Item = Self::Item<'a, K, V>>
    where
        Self: 'a;
    /// returns an iterator over the vertices in the store.
    fn iter(&self) -> Self::Iter<'_>;
}

/*
 ************* Implementations *************
*/

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;
    use alloc::collections::btree_map::{self, BTreeMap};

    impl<K, V> RawStore<K, V> for BTreeMap<K, V>
    where
        K: Ord,
    {
        seal!();
    }

    impl<K, V> Store<K, V> for BTreeMap<K, V>
    where
        K: Ord,
    {
        type Entry<'a>
            = btree_map::Entry<'a, K, V>
        where
            Self: 'a;

        fn entry(&mut self, key: K) -> Self::Entry<'_> {
            self.entry(key)
        }
        fn insert(&mut self, key: K, value: V) -> Option<V> {
            self.insert(key, value)
        }
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::*;
    use core::hash::{BuildHasher, Hash};
    use std::collections::hash_map::{self, HashMap};

    impl<K, V, S> RawStore<K, V> for HashMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        seal!();
    }

    impl<K, V, S> Store<K, V> for HashMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        type Entry<'a>
            = hash_map::Entry<'a, K, V>
        where
            Self: 'a;

        fn entry(&mut self, key: K) -> Self::Entry<'_> {
            self.entry(key)
        }
        fn insert(&mut self, key: K, value: V) -> Option<V> {
            self.insert(key, value)
        }
    }
}
