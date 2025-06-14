/*
    appellation: key_value <module>
    authors: @FL03
*/

/// A [`RawEntry`] represents a single record within a key-value store, providing access to the
/// key alongisde methods for accessing and manipulating the value of the entry.
pub trait RawEntry<'a> {
    type Key;
    type Value;

    private!();
    /// Returns a reference to the key of the entry.
    fn key(&self) -> &Self::Key;
    /// Returns a reference to the value of the entry.
    fn value(&self) -> Option<&Self::Value>;
    /// Returns a mutable reference to the value of the entry.
    fn value_mut(&mut self) -> Option<&mut Self::Value>;
}
/// The [`Entry`] trait extends the [`RawEntry`] trait to provide additional methods for
///
pub trait Entry<'a>: RawEntry<'a> {
    /// if the entry is vacant, insert the given value
    fn or_insert(self, value: Self::Value) -> &'a mut Self::Value;
    /// if the entry does not exist, insert the value returned by the provided function and
    /// return a mutable reference to it.
    fn or_insert_with<F>(self, f: F) -> &'a mut Self::Value
    where
        F: FnOnce() -> Self::Value;
}
/*
 ************* Implementations *************
*/
#[cfg(feature = "alloc")]
macro_rules! entry {
    ($($prefix:ident)::* -> $call:ident($($arg:tt),*)) => {
        $($prefix)::*::Entry::$call($($arg),*)
    };
}
#[cfg(feature = "alloc")]
macro_rules! impl_entry {
    (@raw $($prefix:ident)::* $(where $($preds:tt)*)?) => {
        impl<'a, K, V> RawEntry<'a> for $($prefix)::*::Entry<'a, K, V> $(where $($preds)*)? {
            type Key = K;
            type Value = V;

            seal!();

            fn key(&self) -> &Self::Key {
                entry!($($prefix)::* -> key(self))
            }

            fn value(&self) -> Option<&Self::Value> {
                match self {
                    $($prefix)::*::Entry::Occupied(entry) => Some(entry.get()),
                    $($prefix)::*::Entry::Vacant(_) => None,
                }
            }

            fn value_mut(&mut self) -> Option<&mut Self::Value> {
                match self {
                    $($prefix)::*::Entry::Occupied(entry) => Some(entry.get_mut()),
                    $($prefix)::*::Entry::Vacant(_) => None,
                }
            }
        }

    };
    (@entry $($prefix:ident)::* $(where $($preds:tt)*)?) => {
        impl<'a, K, V> Entry<'a> for $($prefix)::*::Entry<'a, K, V> $(where $($preds)*)? {
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
    (@impl $($rest:tt)*) => {
        impl_entry!(@raw $($rest)*);
        impl_entry!(@entry $($rest)*);

    };
    ($($rest:tt)*) => {
        impl_entry!(@impl $($rest)*);
    };
}

#[cfg(feature = "alloc")]
impl_entry! {
    alloc::collections::btree_map where K: Ord
}

#[cfg(feature = "std")]
impl_entry! {
    std::collections::hash_map where K: Eq + core::hash::Hash
}
