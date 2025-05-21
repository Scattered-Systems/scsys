/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[allow(unused_imports)]
pub use self::{alloc::*, std::*};

mod alloc;
mod std;

/// Compare two types
pub fn type_of<U, V>() -> bool
where
    U: core::any::Any + ?Sized,
    V: core::any::Any + ?Sized,
{
    core::any::TypeId::of::<U>() == core::any::TypeId::of::<V>()
}
