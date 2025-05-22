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

pub trait TypeOf {
    fn of<T: 'static>() -> bool;
}

pub trait IsType {
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
pub trait DType: Any {
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
impl<T> TypeOf for T
where
    T: Any,
{
    fn of<U: 'static>() -> bool {
        type_of::<T, U>()
    }
}

impl<T: 'static> IsType for T {}

impl dyn DType {}
