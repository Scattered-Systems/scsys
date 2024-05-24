/*
    Appellation: dtype <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait OfType {
    fn of<T>() -> bool
    where
        T: 'static,
        Self: 'static,
    {
        core::any::TypeId::of::<T>() == core::any::TypeId::of::<Self>()
    }
}

/*
 ************* Implementations *************
*/
impl<T> OfType for T {}