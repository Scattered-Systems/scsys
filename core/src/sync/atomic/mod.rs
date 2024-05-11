/*
    Appellation: atomic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::order::AtomicOrder;

pub(crate) mod order;

pub(crate) mod prelude {
    pub use super::order::AtomicOrder;
}

#[cfg(test)]
mod tests {}
