/*
    Appellation: scsys-config <library>
    Contrib: @FL03
*/
//! A collection of common configuration primitives and utilities used throughout the
//! [scsys-io](https://scsys.io) ecosystem.
//!
#![crate_type = "lib"]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/favicon.ico"
)]

#[doc(no_inline)]
#[cfg(feature = "config")]
pub use config;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod config;
    #[macro_use]
    pub mod seal;
}

#[doc(inline)]
pub use self::{
    consts::*, error::*, services::prelude::*, traits::prelude::*, types::prelude::*,
    utils::prelude::*,
};
/// the `error` module defines the error-handling routines for the crate
pub mod error;
pub mod services;

/// constants declared for the configuration module
pub mod consts {
    /// The default environment variable prefix used for configuration schemas.
    pub const DEFAULT_ENV_PREFIX: &str = "APP";
    /// The default environment variable separator used for configuration schemas.
    pub const DEFAULT_ENV_SEPARATOR: &str = "_";
    /// The standard artifacts directory name
    pub const ARTIFACTS: &str = ".artifacts";
    /// The standard cache directory name
    pub const CACHE_DIR: &str = ".cache";
    /// The standard configuration directory name
    pub const CONFIG_DIR: &str = ".config";
}

/// this module is used to implement various traits supporting the configuration module
pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod configure;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::configure::*;
    }
}
/// various types used throughout the configuration module
pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod environment;
    pub mod log_level;
    pub mod mode;
    pub mod network_address;
    pub mod scope;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::environment::*;
        #[doc(inline)]
        pub use super::log_level::*;
        #[doc(inline)]
        pub use super::mode::*;
        #[doc(inline)]
        pub use super::network_address::*;
        #[doc(inline)]
        pub use super::scope::*;
    }
}

pub mod utils {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod fs;
    pub mod vars;

    pub(crate) mod prelude {
        #[allow(unused_imports)]
        #[doc(inline)]
        pub use super::fs::*;
        #[doc(inline)]
        pub use super::vars::*;
    }
}

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::consts::*;
    #[doc(no_inline)]
    pub use crate::error::*;
    #[doc(no_inline)]
    pub use crate::services::prelude::*;
    #[doc(no_inline)]
    pub use crate::types::prelude::*;
    #[doc(no_inline)]
    pub use crate::utils::prelude::*;
}
