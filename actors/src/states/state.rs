/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::marker::PhantomData;

pub trait Stateful<T> {
    type Q: ?Sized;
}

pub trait RawState<T> {
    type Data;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Binary {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Ternary {}

impl<T> RawState<T> for Binary {
    type Data = bool;
}

impl<T> RawState<T> for Ternary {
    type Data = (bool, bool);
}

pub type BinaryState<T> = State<Binary, T>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(C)]
pub struct State<Q: ?Sized, T> {
    data: T,
    _state: PhantomData<Q>,
}

impl<Q, T> State<Q, T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            _state: PhantomData::<Q>,
        }
    }

    pub const fn get(&self) -> &T {
        &self.data
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn set(&mut self, data: T) {
        self.data = data;
    }

    pub fn cast<R: ?Sized>(self) -> State<R, T> {
        State {
            data: self.data,
            _state: PhantomData::<R>,
        }
    }
}

impl<T> State<Binary, T> {
    pub fn binary(data: T) -> Self {
        Self {
            data,
            _state: PhantomData::<Binary>,
        }
    }

}

impl<Q, T> core::borrow::Borrow<T> for State<Q, T> {
    fn borrow(&self) -> &T {
        &self.data
    }
}

impl<Q, T> core::borrow::BorrowMut<T> for State<Q, T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.data
    }
}
