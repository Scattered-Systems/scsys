/*
    Appellation: dtype <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::type_of;
use core::any::{Any, TypeId};

pub trait IsType: 'static {
    fn of<T: 'static>() -> bool {
        type_of::<Self, T>()
    }

    fn is<T: 'static>(&self) -> bool {
        type_of::<Self, T>()
    }
}

/// [DType] provides additional information regarding the current type
pub trait DType: Any {
    fn type_id(&self) -> TypeId {
        Any::type_id(self)
    }

    fn type_name(&self) -> &'static str {
        core::any::type_name::<Self>()
    }
}

pub trait DTypeExt: DType {
    fn of<T: 'static>() -> bool {
        type_of::<Self, T>()
    }

    fn is<T: 'static>(&self) -> bool {
        type_of::<Self, T>()
    }

    fn type_id(&self) -> TypeId {
        Any::type_id(self)
    }

    fn type_name<T>() -> &'static str
    where
        T: ?Sized,
    {
        core::any::type_name::<T>()
    }
}

/*
 ************* Implementations *************
*/
impl<T: 'static> IsType for T {}

impl dyn DType {}
