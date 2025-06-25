/*
    Appellation: dtype <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::any::{Any, TypeId};

/// Compare two types
fn type_of<U, V>() -> bool
where
    U: core::any::Any + ?Sized,
    V: core::any::Any + ?Sized,
{
    core::any::TypeId::of::<U>() == core::any::TypeId::of::<V>()
}
/// The [`TypeOf`] trait automatically provides a way to check if a type is of a specific type
/// at compile time. This is useful for generic programming and type checking.
pub trait TypeOf {
    private!();

    fn of<T: 'static>() -> bool;
}
/// The [`IsType`] trait provides a method to check if the current type is of a specific type
/// at runtime. This is useful for dynamic type checking in scenarios where the type may not be
/// known at compile time.
pub trait IsType {
    private!();

    fn is<T>(&self) -> bool
    where
        T: 'static,
        Self: 'static,
    {
        type_of::<Self, T>()
    }
}

/// [`DType`] is a trait designed to provide additional information regarding the type of a
/// particular value.
pub trait DType: 'static + IsType + TypeOf {
    private!();

    fn type_id(&self) -> TypeId {
        Any::type_id(self)
    }

    fn type_name(&self) -> &'static str {
        core::any::type_name::<Self>()
    }
}

/*
 ************* Implementations *************
*/
impl<T> DType for T
where
    T: 'static,
{
    seal!();

    fn type_id(&self) -> TypeId {
        Any::type_id(self)
    }

    fn type_name(&self) -> &'static str {
        core::any::type_name::<Self>()
    }
}

impl<T> TypeOf for T
where
    T: 'static,
{
    seal!();

    fn of<U: 'static>() -> bool {
        type_of::<T, U>()
    }
}

impl<T> IsType for T
where
    T: 'static,
{
    seal!();
}
