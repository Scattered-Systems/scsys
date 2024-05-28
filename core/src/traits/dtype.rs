/*
    Appellation: dtype <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::type_of;
use core::any::{Any, TypeId};

/// [DType] provides additional information regarding the current type
pub trait DType: Any {
    fn of<T: 'static>() -> bool {
        type_of::<Self, T>()
    }

    fn is<T: 'static>(&self) -> bool {
        type_of::<Self, T>()
    }

    fn type_id(&self) -> TypeId {
        Any::type_id(self)
    }

    fn type_name<T>() -> &'static str where T: ?Sized {
        core::any::type_name::<T>()
    }

}

/*
 ************* Implementations *************
*/
impl DType for dyn Any {}
