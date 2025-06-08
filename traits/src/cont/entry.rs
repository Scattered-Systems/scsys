/*
    appellation: entry <module>
    authors: @FL03
*/

/// The [`Entry`] trait seeks to establish a common interface for all elements within a
/// so-called _keyed_ container; i.e. on in which elements can be accessed by any type of key.
pub trait Entry<'a> {
    type Key;
    type Value;
    /// returns a reference to the key of the entry.
    fn key(&self) -> &Self::Key;
    /// if the entry does not exist, insert the provided value and return a mutable reference
    /// to it
    fn or_insert(self, default: Self::Value) -> &'a mut Self::Value;
    /// if the entry does not exist, insert the value returned by the provided function and
    /// return a mutable reference to it.
    fn or_insert_with<F>(self, f: F) -> &'a mut Self::Value
    where
        F: FnOnce() -> Self::Value;
}
/// The [`OrInsert`] trait is a convenience trait that allows for the insertion of some value
/// whenever the entry does not already exist within the container.
pub trait OrInsert<K, V> {
    fn or_insert(&mut self, key: K, value: V) -> V;
}

/*
 ************* Implementations *************
*/

#[allow(unused_macros)]
macro_rules! entry {
    ($($prefix:ident)::* -> $call:ident($($arg:tt),*)) => {
        $($prefix)::*::Entry::$call($($arg),*)
    };
}

#[allow(unused_macros)]
macro_rules! impl_entry {
    ($($prefix:ident)::* $(where $($preds:tt)*)?) => {

        impl<'a, K, V> Entry<'a> for $($prefix)::*::Entry<'a, K, V> $(where $($preds)*)? {
            type Key = K;
            type Value = V;

            fn key(&self) -> &Self::Key {
                entry!($($prefix)::* -> key(self))
            }

            fn or_insert(self, default: Self::Value) -> &'a mut Self::Value {
                entry!($($prefix)::* -> or_insert(self, default))
            }

            fn or_insert_with<F>(self, f: F) -> &'a mut Self::Value
            where
                F: FnOnce() -> Self::Value,
            {
                entry!($($prefix)::* -> or_insert_with(self, f))
            }
        }

    };
}

#[cfg(feature = "alloc")]
impl_entry!(alloc::collections::btree_map where K: Ord);
#[cfg(feature = "std")]
impl_entry!(std::collections::hash_map where K: Eq + core::hash::Hash);
