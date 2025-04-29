/*
    Appellation: stores <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod kv;

pub trait OrInsert<K, V> {
    fn or_insert(&mut self, key: K, value: V) -> &mut V;
}

pub(crate) mod prelude {
    pub use super::OrInsert;
    pub use super::kv::Store as KeyValue;
}
