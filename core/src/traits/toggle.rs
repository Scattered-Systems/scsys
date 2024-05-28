/*
    Appellation: toggle <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::any::Any;

/// Typically, [TypeTag] is used for uninitaliziable `enums` with no variants
pub trait TypeTag: 'static {
    fn is<T>(&self) -> bool where T: Any + ?Sized {
        crate::type_of::<Self, T>()
    }
}

pub trait BinaryTag: TypeTag {
    const MODE: bool;
}

/*
 ************* Implementations *************
*/

impl<T> TypeTag for T where T: 'static {}