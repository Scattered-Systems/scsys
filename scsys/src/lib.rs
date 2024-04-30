/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
//! # scsys
//!
//!
#[doc(inline)]
pub use scsys_core::*;

#[cfg(feature = "actors")]
#[doc(inline)]
pub use scsys_actors as actors;
#[cfg(feature = "derive")]
#[doc(inline)]
pub use scsys_derive::*;
#[cfg(feature = "macros")]
#[doc(inline)]
pub use scsys_macros::*;
#[cfg(feature = "stores")]
#[doc(inline)]
pub use scsys_stores as stores;

// #66 - Cleanup the prelude module(s)
pub mod prelude {
    #[cfg(feature = "actors")]
    #[doc(inline)]
    pub use scsys_actors::prelude::*;
    #[doc(inline)]
    pub use scsys_core::prelude::*;
    #[cfg(feature = "derive")]
    pub use scsys_derive::*;
    #[cfg(feature = "macros")]
    pub use scsys_macros::*;
    #[cfg(feature = "stores")]
    #[doc(inline)]
    pub use scsys_stores::prelude::*;
}
