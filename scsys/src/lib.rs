/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[doc(inline)]
#[cfg(feature = "core")]
pub use scsys_core::*;
#[cfg(feature = "derive")]
pub use scsys_derive::*;
#[cfg(feature = "macros")]
pub use scsys_macros::*;

#[cfg(feature = "bson")]
pub use bson;
#[cfg(feature = "chrono")]
pub use chrono;
#[cfg(feature = "config")]
pub use config;

pub mod prelude {
    #[cfg(feature = "bson")]
    pub use bson;
    #[cfg(feature = "chrono")]
    pub use chrono;
    #[cfg(feature = "config")]
    pub use config;

    #[cfg(feature = "core")]
    pub use super::{
        clients::*, extract::*, generate::*, handlers::*, loggers::*, models::*, networking::*,
        providers::*, schemas::*, states::*,
    };
    #[cfg(feature = "derive")]
    pub use scsys_derive::*;
    #[cfg(feature = "macros")]
    pub use scsys_macros::*;
}
