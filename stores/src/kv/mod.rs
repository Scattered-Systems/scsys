/*
    Appellation: kv <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Key-Value Store

pub trait KeyValue<K, V> {

    fn get(&self, key: &K) -> Option<&V>;

    fn insert(&mut self, key: K, value: V) -> Option<V>;

    fn remove(&mut self, key: &K) -> Option<V>;
}

pub trait OrInsert<K, V> {
    fn or_insert(&mut self, key: K, value: V) -> &mut V;
}

macro_rules! impl_kv {
    ($t:ty, where $($preds:tt)* ) => {

        impl<K, V> KeyValue<K, V> for $t where $($preds)* {
            fn get(&self, key: &K) -> Option<&V> {
                <$t>::get(self, &key)
            }

            fn insert(&mut self, key: K, value: V) -> Option<V> {
                <$t>::insert(self, key, value)
            }

            fn remove(&mut self, key: &K) -> Option<V> {
                <$t>::remove(self, &key)
            }
        }
    };
}

use std::collections::{BTreeMap, HashMap};

impl_kv!(BTreeMap<K, V>, where K: std::cmp::Ord);
impl_kv!(HashMap<K, V>, where K: std::cmp::Eq + std::hash::Hash);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");

        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
        assert_eq!(map.get(&3), Some(&"three"));

        assert_eq!(map.remove(&1), Some("one"));
        assert_eq!(map.remove(&2), Some("two"));
        assert_eq!(map.remove(&3), Some("three"));

        assert_eq!(map.get(&1), None);
        assert_eq!(map.get(&2), None);
        assert_eq!(map.get(&3), None);
    }
}