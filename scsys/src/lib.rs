/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[doc(inline)]
#[cfg(feature = "core")]
pub use scsys_core::*;

pub mod prelude {
    #[cfg(feature = "core")]
    pub use scsys_core::prelude::*;
}
