/*
    Appellation: toggle <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// Typically, [TypeTag] is used for uninitaliziable `enums` with no variants
pub trait TypeTag: 'static {}

#[doc(hidden)]
pub trait Context {
    type Elem;
}

/*
 ************* Implementations *************
*/

impl dyn TypeTag {}

impl<T> dyn Context<Elem = T> {}