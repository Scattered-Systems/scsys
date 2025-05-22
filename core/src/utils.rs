/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "alloc")]
pub use self::alloc::*;
#[cfg(feature = "std")]
pub use self::std::*;

mod alloc;
mod std;
