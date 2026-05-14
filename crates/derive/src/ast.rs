/*
    appellation: ast <module>
    authors: @FL03
*/
#[allow(unused_imports)]
#[doc(inline)]
pub use self::prelude::*;

pub mod getter;

#[allow(unused_imports)]
pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::getter::*;
}
