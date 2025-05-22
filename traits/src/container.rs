/*
    Appellation: container <module>
    Contrib: @FL03
*/
/// [`RawContainer`] defines a standard interface for all _containers_ that are used to store 
/// other entities.
pub unsafe trait RawContainer<T> {
    type Cont<V>;

    private!();
}

/*
 ************* Implementations *************
*/

macro_rules! impl_raw_container {
    ($($($t:ident)::*<$T:ident>),* $(,)?) => {
        $(
            impl_raw_container!(@impl $($t)::*<$T>);
        )*
    };
    (@impl $($t:ident)::*<$T:ident>) => {
        unsafe impl<$T> RawContainer<T> for $($t)::*<$T> {
            type Cont<V> = $($t)::*<V>;

            seal!();
        }
    };
}

#[cfg(feature = "alloc")]
impl_raw_container! {
    alloc::vec::Vec<T>,
    alloc::boxed::Box<T>,
    alloc::rc::Rc<T>,
    alloc::sync::Arc<T>,
    alloc::collections::BTreeSet<T>,
    alloc::collections::LinkedList<T>,
}
