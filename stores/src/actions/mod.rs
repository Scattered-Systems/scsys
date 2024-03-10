/*
    Appellation: actions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod crud;

pub(crate) mod prelude {
    pub use super::crud::CRUD;
}

#[cfg(test)]
mod tests {}