/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::marker::PhantomData;

/// [State] is an abstract object that allows a particular _kind_ of state to be associated
/// with some data.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct State<K, V> {
    pub(crate) data: V,
    pub(crate) _state: PhantomData<K>,
}

impl<K, V> State<K, V> {
    pub fn new(data: V) -> Self {
        Self {
            data,
            _state: PhantomData::<K>,
        }
    }

    pub fn data(&self) -> &V {
        &self.data
    }

    pub fn is_state<R: 'static>(&self) -> bool
    where
        K: 'static,
    {
        use core::any::TypeId;
        TypeId::of::<PhantomData<K>>() == TypeId::of::<PhantomData<R>>()
    }
}
