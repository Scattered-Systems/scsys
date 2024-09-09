/*
    Appellation: binary <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::State;

pub enum Unary {}

pub enum Binary {}

pub enum Ternary {}

pub enum Nary<const N: usize> {}

pub type BinaryState<T> = State<Binary, T>;

pub type UnaryState<T> = State<Unary, T>;

pub type NState<T, const N: usize = 4> = State<Nary<N>, T>;
