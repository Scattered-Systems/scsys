/*
    Appellation: errors <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::error::*;

mod error;

pub(crate) mod primitives {

    /// Type alias for a boxed standard error
    pub type BaseError = Box<dyn std::error::Error>;
    /// Type alias for a boxed error with send, sync, and static flags enabled
    pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
}
