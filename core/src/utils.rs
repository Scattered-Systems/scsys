/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "alloc")]
pub use self::alloc::*;
#[cfg(feature = "std")]
pub use self::std::*;

#[cfg(feature = "alloc")]
mod alloc;
#[cfg(feature = "std")]
mod std;

/// Compare two types
pub fn type_of<U, V>() -> bool
where
    U: core::any::Any + ?Sized,
    V: core::any::Any + ?Sized,
{
    core::any::TypeId::of::<U>() == core::any::TypeId::of::<V>()
}
