/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#![allow(unused_imports)]

#[doc(inline)]
#[cfg(feature = "core")]
pub use scsys_core as core;
#[cfg(feature = "derive")]
pub use scsys_derive::*;
#[cfg(feature = "macros")]
pub use scsys_macros::*;

pub mod prelude {
    #[cfg(feature = "bson")]
    pub use bson;
    #[cfg(feature = "chrono")]
    pub use chrono;
    #[cfg(feature = "config")]
    pub use config;
    #[cfg(feature = "rand")]
    pub use rand;

    #[cfg(feature = "core")]
    pub use super::core::{
        self,
        accounts::{Account, AccountSpec},
        extract::{Extractor, ExtractorSpec, FileExtSpec, FileExtractor, FileInterface},
        generate::{
            generate_random_number, generate_random_string, DigitGenerator, StringGenerator,
        },
        logging::{logger_from_env, Logger, LoggerSpec},
        models::*,
        networking::{ReverseProxy, Server},
        providers::{Cache, Database, Provider, Web3Provider},
        schemas::*,
        states::{State, Stateful},
    };
}
