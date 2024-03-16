/*
    Appellation: atomic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::order::AtomicOrder;

pub(crate) mod order;

#[cfg(test)]
mod tests {}
