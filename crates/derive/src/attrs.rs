/*
    appellation: attrs <module>
    authors: @FL03
*/
#[doc(inline)]
pub use self::prelude::*;

pub mod display_attrs;
pub mod nested;
pub mod root;
pub mod variants;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::display_attrs::*;
    #[doc(inline)]
    pub use super::nested::*;
    #[doc(inline)]
    pub use super::root::*;
    #[doc(inline)]
    pub use super::variants::*;
}
